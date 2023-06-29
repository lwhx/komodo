//

use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    entities::{
        deployment::{Deployment, PartialDeploymentConfig, TerminationSignal},
        update::Update,
    },
    MongoDocument,
};

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Deployment)]
pub struct GetDeployment {
    pub id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<Deployment>)]
pub struct ListDeployments {
    pub query: Option<MongoDocument>,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Deployment)]
pub struct CreateDeployment {
    pub name: String,
    pub config: PartialDeploymentConfig,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Deployment)]
pub struct DeleteDeployment {
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Deployment)]
pub struct UpdateDeployment {
    pub id: String,
    pub config: PartialDeploymentConfig,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct RenameDeployment {
    pub id: String,
    pub name: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct Deploy {
    pub deployment_id: String,
    pub stop_signal: Option<TerminationSignal>,
    pub stop_time: Option<i32>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct StartContainer {
    pub deployment_id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct StopContainer {
    pub deployment_id: String,
    pub signal: Option<TerminationSignal>,
    pub time: Option<i32>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct RemoveContainer {
    pub deployment_id: String,
    pub signal: Option<TerminationSignal>,
    pub time: Option<i32>,
}
