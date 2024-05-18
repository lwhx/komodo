use monitor_client::entities::{
  server::docker_network::DockerNetwork, update::Log,
};
use periphery_client::api::network::{
  CreateNetwork, DeleteNetwork, GetNetworkList, PruneNetworks,
};
use resolver_api::Resolve;

use crate::{
  helpers::docker::{self, client::docker_client},
  State,
};

//

impl Resolve<GetNetworkList> for State {
  #[instrument(name = "GetNetworkList", level = "debug", skip(self))]
  async fn resolve(
    &self,
    _: GetNetworkList,
    _: (),
  ) -> anyhow::Result<Vec<DockerNetwork>> {
    docker_client().list_networks().await
  }
}

//

impl Resolve<CreateNetwork> for State {
  #[instrument(name = "CreateNetwork", skip(self))]
  async fn resolve(
    &self,
    CreateNetwork { name, driver }: CreateNetwork,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(docker::network::create_network(&name, driver).await)
  }
}

//

impl Resolve<DeleteNetwork> for State {
  #[instrument(name = "DeleteNetwork", skip(self))]
  async fn resolve(
    &self,
    DeleteNetwork { name }: DeleteNetwork,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(docker::network::delete_network(&name).await)
  }
}

//

impl Resolve<PruneNetworks> for State {
  #[instrument(name = "PruneNetworks", skip(self))]
  async fn resolve(
    &self,
    _: PruneNetworks,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(docker::network::prune_networks().await)
  }
}
