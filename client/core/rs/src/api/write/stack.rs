use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  stack::{Stack, _PartialStackConfig}, update::Update, NoData
};

use super::MonitorWriteRequest;

//

/// Create a stack. Response: [Stack].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Stack)]
pub struct CreateStack {
  /// The name given to newly created stack.
  pub name: String,
  /// Optional partial config to initialize the stack with.
  pub config: _PartialStackConfig,
}

//

/// Creates a new stack with given `name` and the configuration
/// of the stack at the given `id`. Response: [Stack].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Stack)]
pub struct CopyStack {
  /// The name of the new stack.
  pub name: String,
  /// The id of the stack to copy.
  pub id: String,
}

//

/// Deletes the stack at the given id, and returns the deleted stack.
/// Response: [Stack]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Stack)]
pub struct DeleteStack {
  /// The id or name of the stack to delete.
  pub id: String,
}

//

/// Update the stack at the given id, and return the updated stack.
/// Response: [Stack].
///
/// Note. If the attached server for the stack changes,
/// the stack will be deleted / cleaned up on the old server.
///
/// Note. This method updates only the fields which are set in the [_PartialStackConfig],
/// merging diffs into the final document.
/// This is helpful when multiple users are using
/// the same resources concurrently by ensuring no unintentional
/// field changes occur from out of date local state.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Stack)]
pub struct UpdateStack {
  /// The id of the Stack to update.
  pub id: String,
  /// The partial config update to apply.
  pub config: _PartialStackConfig,
}

//

/// Rename the stack at id to the given name. Response: [Update].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Update)]
pub struct RenameStack {
  /// The id of the stack to rename.
  pub id: String,
  /// The new name.
  pub name: String,
}

//

/// Trigger a refresh of the cached compose file contents.
/// Refreshes:
///   - Whether the remote file is missing
///   - The latest json, and for repos, the remote contents, hash, and message.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(NoData)]
pub struct RefreshStackCache {
  /// Id or name
  pub stack: String,
}

//

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StackWebhookAction {
  Refresh,
  Deploy,
}

/// Create a webhook on the github repo attached to the stack
/// passed in request. Response: [CreateStackWebhookResponse]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CreateStackWebhookResponse)]
pub struct CreateStackWebhook {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub stack: String,
  /// "Refresh" or "Deploy"
  pub action: StackWebhookAction,
}

#[typeshare]
pub type CreateStackWebhookResponse = NoData;

//

/// Delete the webhook on the github repo attached to the stack
/// passed in request. Response: [DeleteStackWebhookResponse]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(DeleteStackWebhookResponse)]
pub struct DeleteStackWebhook {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub stack: String,
  /// "Refresh" or "Deploy"
  pub action: StackWebhookAction,
}

#[typeshare]
pub type DeleteStackWebhookResponse = NoData;