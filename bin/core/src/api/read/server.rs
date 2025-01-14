use std::{
  cmp,
  collections::HashMap,
  sync::{Arc, OnceLock},
};

use anyhow::{anyhow, Context};
use async_timing_util::{
  get_timelength_in_ms, unix_timestamp_ms, FIFTEEN_SECONDS_MS,
};
use komodo_client::{
  api::read::*,
  entities::{
    deployment::Deployment,
    docker::{
      container::{Container, ContainerListItem},
      image::{Image, ImageHistoryResponseItem},
      network::Network,
      volume::Volume,
    },
    permission::PermissionLevel,
    server::{
      Server, ServerActionState, ServerListItem, ServerState,
    },
    stack::{Stack, StackServiceNames},
    update::Log,
    user::User,
    ResourceTarget,
  },
};
use mungos::{
  find::find_collect,
  mongodb::{bson::doc, options::FindOptions},
};
use periphery_client::api::{
  self as periphery,
  container::InspectContainer,
  image::{ImageHistory, InspectImage},
  network::InspectNetwork,
  volume::InspectVolume,
};
use resolver_api::{Resolve, ResolveToString};
use tokio::sync::Mutex;

use crate::{
  helpers::{periphery_client, query::get_all_tags},
  resource,
  stack::compose_container_match_regex,
  state::{action_states, db_client, server_status_cache, State},
};

impl Resolve<GetServersSummary, User> for State {
  async fn resolve(
    &self,
    GetServersSummary {}: GetServersSummary,
    user: User,
  ) -> anyhow::Result<GetServersSummaryResponse> {
    let servers = resource::list_for_user::<Server>(
      Default::default(),
      &user,
      &[],
    )
    .await?;
    let mut res = GetServersSummaryResponse::default();
    for server in servers {
      res.total += 1;
      match server.info.state {
        ServerState::Ok => {
          res.healthy += 1;
        }
        ServerState::NotOk => {
          res.unhealthy += 1;
        }
        ServerState::Disabled => {
          res.disabled += 1;
        }
      }
    }
    Ok(res)
  }
}

impl Resolve<GetPeripheryVersion, User> for State {
  async fn resolve(
    &self,
    req: GetPeripheryVersion,
    user: User,
  ) -> anyhow::Result<GetPeripheryVersionResponse> {
    let server = resource::get_check_permissions::<Server>(
      &req.server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let version = server_status_cache()
      .get(&server.id)
      .await
      .map(|s| s.version.clone())
      .unwrap_or(String::from("unknown"));
    Ok(GetPeripheryVersionResponse { version })
  }
}

impl Resolve<GetServer, User> for State {
  async fn resolve(
    &self,
    req: GetServer,
    user: User,
  ) -> anyhow::Result<Server> {
    resource::get_check_permissions::<Server>(
      &req.server,
      &user,
      PermissionLevel::Read,
    )
    .await
  }
}

impl Resolve<ListServers, User> for State {
  async fn resolve(
    &self,
    ListServers { query }: ListServers,
    user: User,
  ) -> anyhow::Result<Vec<ServerListItem>> {
    let all_tags = if query.tags.is_empty() {
      vec![]
    } else {
      get_all_tags(None).await?
    };
    resource::list_for_user::<Server>(query, &user, &all_tags).await
  }
}

impl Resolve<ListFullServers, User> for State {
  async fn resolve(
    &self,
    ListFullServers { query }: ListFullServers,
    user: User,
  ) -> anyhow::Result<ListFullServersResponse> {
    let all_tags = if query.tags.is_empty() {
      vec![]
    } else {
      get_all_tags(None).await?
    };
    resource::list_full_for_user::<Server>(query, &user, &all_tags)
      .await
  }
}

impl Resolve<GetServerState, User> for State {
  async fn resolve(
    &self,
    GetServerState { server }: GetServerState,
    user: User,
  ) -> anyhow::Result<GetServerStateResponse> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let status = server_status_cache()
      .get(&server.id)
      .await
      .ok_or(anyhow!("did not find cached status for server"))?;
    let response = GetServerStateResponse {
      status: status.state,
    };
    Ok(response)
  }
}

impl Resolve<GetServerActionState, User> for State {
  async fn resolve(
    &self,
    GetServerActionState { server }: GetServerActionState,
    user: User,
  ) -> anyhow::Result<ServerActionState> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let action_state = action_states()
      .server
      .get(&server.id)
      .await
      .unwrap_or_default()
      .get()?;
    Ok(action_state)
  }
}

// This protects the peripheries from spam requests
const SYSTEM_INFO_EXPIRY: u128 = FIFTEEN_SECONDS_MS;
type SystemInfoCache = Mutex<HashMap<String, Arc<(String, u128)>>>;
fn system_info_cache() -> &'static SystemInfoCache {
  static SYSTEM_INFO_CACHE: OnceLock<SystemInfoCache> =
    OnceLock::new();
  SYSTEM_INFO_CACHE.get_or_init(Default::default)
}

impl ResolveToString<GetSystemInformation, User> for State {
  async fn resolve_to_string(
    &self,
    GetSystemInformation { server }: GetSystemInformation,
    user: User,
  ) -> anyhow::Result<String> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;

    let mut lock = system_info_cache().lock().await;
    let res = match lock.get(&server.id) {
      Some(cached) if cached.1 > unix_timestamp_ms() => {
        cached.0.clone()
      }
      _ => {
        let stats = periphery_client(&server)?
          .request(periphery::stats::GetSystemInformation {})
          .await?;
        let res = serde_json::to_string(&stats)?;
        lock.insert(
          server.id,
          (res.clone(), unix_timestamp_ms() + SYSTEM_INFO_EXPIRY)
            .into(),
        );
        res
      }
    };
    Ok(res)
  }
}

impl ResolveToString<GetSystemStats, User> for State {
  async fn resolve_to_string(
    &self,
    GetSystemStats { server }: GetSystemStats,
    user: User,
  ) -> anyhow::Result<String> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let status =
      server_status_cache().get(&server.id).await.with_context(
        || format!("did not find status for server at {}", server.id),
      )?;
    let stats = status
      .stats
      .as_ref()
      .context("server stats not available")?;
    let stats = serde_json::to_string(&stats)?;
    Ok(stats)
  }
}

// This protects the peripheries from spam requests
const PROCESSES_EXPIRY: u128 = FIFTEEN_SECONDS_MS;
type ProcessesCache = Mutex<HashMap<String, Arc<(String, u128)>>>;
fn processes_cache() -> &'static ProcessesCache {
  static PROCESSES_CACHE: OnceLock<ProcessesCache> = OnceLock::new();
  PROCESSES_CACHE.get_or_init(Default::default)
}

impl ResolveToString<ListSystemProcesses, User> for State {
  async fn resolve_to_string(
    &self,
    ListSystemProcesses { server }: ListSystemProcesses,
    user: User,
  ) -> anyhow::Result<String> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let mut lock = processes_cache().lock().await;
    let res = match lock.get(&server.id) {
      Some(cached) if cached.1 > unix_timestamp_ms() => {
        cached.0.clone()
      }
      _ => {
        let stats = periphery_client(&server)?
          .request(periphery::stats::GetSystemProcesses {})
          .await?;
        let res = serde_json::to_string(&stats)?;
        lock.insert(
          server.id,
          (res.clone(), unix_timestamp_ms() + PROCESSES_EXPIRY)
            .into(),
        );
        res
      }
    };
    Ok(res)
  }
}

const STATS_PER_PAGE: i64 = 200;

impl Resolve<GetHistoricalServerStats, User> for State {
  async fn resolve(
    &self,
    GetHistoricalServerStats {
      server,
      granularity,
      page,
    }: GetHistoricalServerStats,
    user: User,
  ) -> anyhow::Result<GetHistoricalServerStatsResponse> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let granularity =
      get_timelength_in_ms(granularity.to_string().parse().unwrap())
        as i64;
    let mut ts_vec = Vec::<i64>::new();
    let curr_ts = unix_timestamp_ms() as i64;
    let mut curr_ts = curr_ts
      - curr_ts % granularity
      - granularity * STATS_PER_PAGE * page as i64;
    for _ in 0..STATS_PER_PAGE {
      ts_vec.push(curr_ts);
      curr_ts -= granularity;
    }

    let stats = find_collect(
      &db_client().stats,
      doc! {
        "sid": server.id,
        "ts": { "$in": ts_vec },
      },
      FindOptions::builder()
        .sort(doc! { "ts": -1 })
        .skip(page as u64 * STATS_PER_PAGE as u64)
        .limit(STATS_PER_PAGE)
        .build(),
    )
    .await
    .context("failed to pull stats from db")?;
    let next_page = if stats.len() == STATS_PER_PAGE as usize {
      Some(page + 1)
    } else {
      None
    };
    let res = GetHistoricalServerStatsResponse { stats, next_page };
    Ok(res)
  }
}

impl ResolveToString<ListDockerContainers, User> for State {
  async fn resolve_to_string(
    &self,
    ListDockerContainers { server }: ListDockerContainers,
    user: User,
  ) -> anyhow::Result<String> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let cache = server_status_cache()
      .get_or_insert_default(&server.id)
      .await;
    if let Some(containers) = &cache.containers {
      serde_json::to_string(containers)
        .context("failed to serialize response")
    } else {
      Ok(String::from("[]"))
    }
  }
}

impl Resolve<ListAllDockerContainers, User> for State {
  async fn resolve(
    &self,
    ListAllDockerContainers { servers }: ListAllDockerContainers,
    user: User,
  ) -> anyhow::Result<Vec<ContainerListItem>> {
    let servers = resource::list_for_user::<Server>(
      Default::default(),
      &user,
      &[],
    )
    .await?
    .into_iter()
    .filter(|server| {
      servers.is_empty()
        || servers.contains(&server.id)
        || servers.contains(&server.name)
    });

    let mut containers = Vec::<ContainerListItem>::new();

    for server in servers {
      let cache = server_status_cache()
        .get_or_insert_default(&server.id)
        .await;
      if let Some(more_containers) = &cache.containers {
        containers.extend(more_containers.clone());
      }
    }

    Ok(containers)
  }
}

impl Resolve<InspectDockerContainer, User> for State {
  async fn resolve(
    &self,
    InspectDockerContainer { server, container }: InspectDockerContainer,
    user: User,
  ) -> anyhow::Result<Container> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let cache = server_status_cache()
      .get_or_insert_default(&server.id)
      .await;
    if cache.state != ServerState::Ok {
      return Err(anyhow!(
        "Cannot inspect container: server is {:?}",
        cache.state
      ));
    }
    periphery_client(&server)?
      .request(InspectContainer { name: container })
      .await
  }
}

const MAX_LOG_LENGTH: u64 = 5000;

impl Resolve<GetContainerLog, User> for State {
  async fn resolve(
    &self,
    GetContainerLog {
      server,
      container,
      tail,
      timestamps,
    }: GetContainerLog,
    user: User,
  ) -> anyhow::Result<Log> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    periphery_client(&server)?
      .request(periphery::container::GetContainerLog {
        name: container,
        tail: cmp::min(tail, MAX_LOG_LENGTH),
        timestamps,
      })
      .await
      .context("failed at call to periphery")
  }
}

impl Resolve<SearchContainerLog, User> for State {
  async fn resolve(
    &self,
    SearchContainerLog {
      server,
      container,
      terms,
      combinator,
      invert,
      timestamps,
    }: SearchContainerLog,
    user: User,
  ) -> anyhow::Result<Log> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    periphery_client(&server)?
      .request(periphery::container::GetContainerLogSearch {
        name: container,
        terms,
        combinator,
        invert,
        timestamps,
      })
      .await
      .context("failed at call to periphery")
  }
}

impl Resolve<GetResourceMatchingContainer, User> for State {
  async fn resolve(
    &self,
    GetResourceMatchingContainer { server, container }: GetResourceMatchingContainer,
    user: User,
  ) -> anyhow::Result<GetResourceMatchingContainerResponse> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    // first check deployments
    if let Ok(deployment) =
      resource::get::<Deployment>(&container).await
    {
      return Ok(GetResourceMatchingContainerResponse {
        resource: ResourceTarget::Deployment(deployment.id).into(),
      });
    }

    // then check stacks
    let stacks =
      resource::list_full_for_user_using_document::<Stack>(
        doc! { "config.server_id": &server.id },
        &user,
      )
      .await?;

    // check matching stack
    for stack in stacks {
      for StackServiceNames {
        service_name,
        container_name,
        ..
      } in stack
        .info
        .deployed_services
        .unwrap_or(stack.info.latest_services)
      {
        let is_match = match compose_container_match_regex(&container_name)
          .with_context(|| format!("failed to construct container name matching regex for service {service_name}")) 
        {
          Ok(regex) => regex,
          Err(e) => {
            warn!("{e:#}");
            continue;
          }
        }.is_match(&container);

        if is_match {
          return Ok(GetResourceMatchingContainerResponse {
            resource: ResourceTarget::Stack(stack.id).into(),
          });
        }
      }
    }

    Ok(GetResourceMatchingContainerResponse { resource: None })
  }
}

impl ResolveToString<ListDockerNetworks, User> for State {
  async fn resolve_to_string(
    &self,
    ListDockerNetworks { server }: ListDockerNetworks,
    user: User,
  ) -> anyhow::Result<String> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let cache = server_status_cache()
      .get_or_insert_default(&server.id)
      .await;
    if let Some(networks) = &cache.networks {
      serde_json::to_string(networks)
        .context("failed to serialize response")
    } else {
      Ok(String::from("[]"))
    }
  }
}

impl Resolve<InspectDockerNetwork, User> for State {
  async fn resolve(
    &self,
    InspectDockerNetwork { server, network }: InspectDockerNetwork,
    user: User,
  ) -> anyhow::Result<Network> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let cache = server_status_cache()
      .get_or_insert_default(&server.id)
      .await;
    if cache.state != ServerState::Ok {
      return Err(anyhow!(
        "Cannot inspect network: server is {:?}",
        cache.state
      ));
    }
    periphery_client(&server)?
      .request(InspectNetwork { name: network })
      .await
  }
}

impl ResolveToString<ListDockerImages, User> for State {
  async fn resolve_to_string(
    &self,
    ListDockerImages { server }: ListDockerImages,
    user: User,
  ) -> anyhow::Result<String> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let cache = server_status_cache()
      .get_or_insert_default(&server.id)
      .await;
    if let Some(images) = &cache.images {
      serde_json::to_string(images)
        .context("failed to serialize response")
    } else {
      Ok(String::from("[]"))
    }
  }
}

impl Resolve<InspectDockerImage, User> for State {
  async fn resolve(
    &self,
    InspectDockerImage { server, image }: InspectDockerImage,
    user: User,
  ) -> anyhow::Result<Image> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let cache = server_status_cache()
      .get_or_insert_default(&server.id)
      .await;
    if cache.state != ServerState::Ok {
      return Err(anyhow!(
        "Cannot inspect image: server is {:?}",
        cache.state
      ));
    }
    periphery_client(&server)?
      .request(InspectImage { name: image })
      .await
  }
}

impl Resolve<ListDockerImageHistory, User> for State {
  async fn resolve(
    &self,
    ListDockerImageHistory { server, image }: ListDockerImageHistory,
    user: User,
  ) -> anyhow::Result<Vec<ImageHistoryResponseItem>> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let cache = server_status_cache()
      .get_or_insert_default(&server.id)
      .await;
    if cache.state != ServerState::Ok {
      return Err(anyhow!(
        "Cannot get image history: server is {:?}",
        cache.state
      ));
    }
    periphery_client(&server)?
      .request(ImageHistory { name: image })
      .await
  }
}

impl ResolveToString<ListDockerVolumes, User> for State {
  async fn resolve_to_string(
    &self,
    ListDockerVolumes { server }: ListDockerVolumes,
    user: User,
  ) -> anyhow::Result<String> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let cache = server_status_cache()
      .get_or_insert_default(&server.id)
      .await;
    if let Some(volumes) = &cache.volumes {
      serde_json::to_string(volumes)
        .context("failed to serialize response")
    } else {
      Ok(String::from("[]"))
    }
  }
}

impl Resolve<InspectDockerVolume, User> for State {
  async fn resolve(
    &self,
    InspectDockerVolume { server, volume }: InspectDockerVolume,
    user: User,
  ) -> anyhow::Result<Volume> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let cache = server_status_cache()
      .get_or_insert_default(&server.id)
      .await;
    if cache.state != ServerState::Ok {
      return Err(anyhow!(
        "Cannot inspect volume: server is {:?}",
        cache.state
      ));
    }
    periphery_client(&server)?
      .request(InspectVolume { name: volume })
      .await
  }
}

impl ResolveToString<ListComposeProjects, User> for State {
  async fn resolve_to_string(
    &self,
    ListComposeProjects { server }: ListComposeProjects,
    user: User,
  ) -> anyhow::Result<String> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let cache = server_status_cache()
      .get_or_insert_default(&server.id)
      .await;
    if let Some(projects) = &cache.projects {
      serde_json::to_string(projects)
        .context("failed to serialize response")
    } else {
      Ok(String::from("[]"))
    }
  }
}
