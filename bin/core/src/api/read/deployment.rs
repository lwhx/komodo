use std::{cmp, str::FromStr};

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::read::*,
  entities::{
    deployment::{
      Deployment, DeploymentActionState, DeploymentConfig,
      DeploymentImage, DeploymentListItem, DockerContainerState,
      DockerContainerStats,
    },
    permission::PermissionLevel,
    server::Server,
    update::{Log, ResourceTargetVariant, UpdateStatus},
    user::User,
    Operation,
  },
};
use mungos::{
  find::find_collect,
  mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOneOptions,
  },
};
use periphery_client::api;
use resolver_api::Resolve;

use crate::{
  db::db_client,
  helpers::{
    cache::deployment_status_cache,
    periphery_client,
    resource::{get_resource_ids_for_non_admin, StateResource},
  },
  state::{action_states, State},
};

#[async_trait]
impl Resolve<GetDeployment, User> for State {
  async fn resolve(
    &self,
    GetDeployment { deployment }: GetDeployment,
    user: User,
  ) -> anyhow::Result<Deployment> {
    Deployment::get_resource_check_permissions(
      &deployment,
      &user,
      PermissionLevel::Read,
    )
    .await
  }
}

#[async_trait]
impl Resolve<ListDeployments, User> for State {
  async fn resolve(
    &self,
    ListDeployments { query }: ListDeployments,
    user: User,
  ) -> anyhow::Result<Vec<DeploymentListItem>> {
    Deployment::list_resources_for_user(query, &user).await
  }
}

#[async_trait]
impl Resolve<GetDeploymentStatus, User> for State {
  async fn resolve(
    &self,
    GetDeploymentStatus { deployment }: GetDeploymentStatus,
    user: User,
  ) -> anyhow::Result<GetDeploymentStatusResponse> {
    let deployment = Deployment::get_resource_check_permissions(
      &deployment,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let status = deployment_status_cache()
      .get(&deployment.id)
      .await
      .unwrap_or_default();
    let response = GetDeploymentStatusResponse {
      status: status
        .curr
        .container
        .as_ref()
        .and_then(|c| c.status.clone()),
      state: status.curr.state,
    };
    Ok(response)
  }
}

const MAX_LOG_LENGTH: u64 = 5000;

#[async_trait]
impl Resolve<GetLog, User> for State {
  async fn resolve(
    &self,
    GetLog { deployment, tail }: GetLog,
    user: User,
  ) -> anyhow::Result<Log> {
    let Deployment {
      name,
      config: DeploymentConfig { server_id, .. },
      ..
    } = Deployment::get_resource_check_permissions(
      &deployment,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    if server_id.is_empty() {
      return Ok(Log::default());
    }
    let server = Server::get_resource(&server_id).await?;
    periphery_client(&server)?
      .request(api::container::GetContainerLog {
        name,
        tail: cmp::min(tail, MAX_LOG_LENGTH),
      })
      .await
      .context("failed at call to periphery")
  }
}

#[async_trait]
impl Resolve<SearchLog, User> for State {
  async fn resolve(
    &self,
    SearchLog { deployment, search }: SearchLog,
    user: User,
  ) -> anyhow::Result<Log> {
    let Deployment {
      name,
      config: DeploymentConfig { server_id, .. },
      ..
    } = Deployment::get_resource_check_permissions(
      &deployment,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    if server_id.is_empty() {
      return Ok(Log::default());
    }
    let server = Server::get_resource(&server_id).await?;
    periphery_client(&server)?
      .request(api::container::GetContainerLogSearch { name, search })
      .await
      .context("failed at call to periphery")
  }
}

#[async_trait]
impl Resolve<GetDeployedVersion, User> for State {
  async fn resolve(
    &self,
    GetDeployedVersion { deployment }: GetDeployedVersion,
    user: User,
  ) -> anyhow::Result<GetDeployedVersionResponse> {
    let Deployment {
      id,
      config: DeploymentConfig { image, .. },
      ..
    } = Deployment::get_resource_check_permissions(
      &deployment,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let version = match image {
      DeploymentImage::Build { .. } => {
        let latest_deploy_update = db_client()
          .await
          .updates
          .find_one(
            doc! {
                "target": {
                    "type": "Deployment",
                    "id": id
                },
                "operation": Operation::DeployContainer.to_string(),
                "status": UpdateStatus::Complete.to_string(),
                "success": true,
            },
            FindOneOptions::builder()
              .sort(doc! { "_id": -1 })
              .build(),
          )
          .await
          .context(
            "failed at query to get latest deploy update from mongo",
          )?;
        if let Some(update) = latest_deploy_update {
          if !update.version.is_none() {
            update.version.to_string()
          } else {
            "unknown".to_string()
          }
        } else {
          "unknown".to_string()
        }
      }
      DeploymentImage::Image { image } => {
        let split = image.split(':').collect::<Vec<&str>>();
        if let Some(version) = split.get(1) {
          version.to_string()
        } else {
          "unknown".to_string()
        }
      }
    };
    Ok(GetDeployedVersionResponse { version })
  }
}

#[async_trait]
impl Resolve<GetDeploymentStats, User> for State {
  async fn resolve(
    &self,
    GetDeploymentStats { deployment }: GetDeploymentStats,
    user: User,
  ) -> anyhow::Result<DockerContainerStats> {
    let Deployment {
      name,
      config: DeploymentConfig { server_id, .. },
      ..
    } = Deployment::get_resource_check_permissions(
      &deployment,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    if server_id.is_empty() {
      return Err(anyhow!("deployment has no server attached"));
    }
    let server = Server::get_resource(&server_id).await?;
    periphery_client(&server)?
      .request(api::container::GetContainerStats { name })
      .await
      .context("failed to get stats from periphery")
  }
}

#[async_trait]
impl Resolve<GetDeploymentActionState, User> for State {
  async fn resolve(
    &self,
    GetDeploymentActionState { deployment }: GetDeploymentActionState,
    user: User,
  ) -> anyhow::Result<DeploymentActionState> {
    let deployment = Deployment::get_resource_check_permissions(
      &deployment,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let action_state = action_states()
      .deployment
      .get(&deployment.id)
      .await
      .unwrap_or_default();
    Ok(action_state)
  }
}

#[async_trait]
impl Resolve<GetDeploymentsSummary, User> for State {
  async fn resolve(
    &self,
    GetDeploymentsSummary {}: GetDeploymentsSummary,
    user: User,
  ) -> anyhow::Result<GetDeploymentsSummaryResponse> {
    let query = if user.admin {
      None
    } else {
      let ids = get_resource_ids_for_non_admin(
        &user.id,
        ResourceTargetVariant::Deployment,
      )
      .await?
      .into_iter()
      .flat_map(|id| ObjectId::from_str(&id))
      .collect::<Vec<_>>();
      let query = doc! {
        "_id": { "$in": ids }
      };
      Some(query)
    };

    let deployments =
      find_collect(&db_client().await.deployments, query, None)
        .await
        .context("failed to count all deployment documents")?;
    let mut res = GetDeploymentsSummaryResponse::default();
    let status_cache = deployment_status_cache();
    for deployment in deployments {
      res.total += 1;
      let status =
        status_cache.get(&deployment.id).await.unwrap_or_default();
      match status.curr.state {
        DockerContainerState::Running => {
          res.running += 1;
        }
        DockerContainerState::Unknown => {
          res.unknown += 1;
        }
        DockerContainerState::NotDeployed => {
          res.not_deployed += 1;
        }
        _ => {
          res.stopped += 1;
        }
      }
    }
    Ok(res)
  }
}
