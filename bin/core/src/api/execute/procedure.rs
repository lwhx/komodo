use std::pin::Pin;

use monitor_client::{
  api::execute::RunProcedure,
  entities::{
    permission::PermissionLevel, procedure::Procedure,
    update::Update, user::User, Operation,
  },
};
use mungos::{by_id::update_one_by_id, mongodb::bson::to_document};
use resolver_api::Resolve;
use serror::serialize_error_pretty;
use tokio::sync::Mutex;

use crate::{
  helpers::{
    procedure::execute_procedure,
    update::{add_update, make_update, update_update},
  },
  resource::{self, refresh_procedure_state_cache},
  state::{action_states, db_client, State},
};

impl Resolve<RunProcedure, User> for State {
  #[instrument(name = "RunProcedure", skip(self, user))]
  async fn resolve(
    &self,
    RunProcedure { procedure }: RunProcedure,
    user: User,
  ) -> anyhow::Result<Update> {
    resolve_inner(procedure, user).await
  }
}

fn resolve_inner(
  procedure: String,
  user: User,
) -> Pin<
  Box<
    dyn std::future::Future<Output = anyhow::Result<Update>> + Send,
  >,
> {
  Box::pin(async move {
    let procedure = resource::get_check_permissions::<Procedure>(
      &procedure,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // get the action state for the procedure (or insert default).
    let action_state = action_states()
      .procedure
      .get_or_insert_default(&procedure.id)
      .await;

    // This will set action state back to default when dropped.
    // Will also check to ensure procedure not already busy before updating.
    let _action_guard =
      action_state.update(|state| state.running = true)?;

    let mut update =
      make_update(&procedure, Operation::RunProcedure, &user);
    update.in_progress();
    update.push_simple_log(
      "execute procedure",
      format!("Executing procedure: {}", procedure.name),
    );

    update.id = add_update(update.clone()).await?;

    let update = Mutex::new(update);

    let res = execute_procedure(&procedure, &update).await;

    let mut update = update.into_inner();

    match res {
      Ok(_) => {
        update.push_simple_log(
          "execution ok",
          "the procedure has completed with no errors",
        );
      }
      Err(e) => update.push_error_log(
        "execution error",
        serialize_error_pretty(&e),
      ),
    }

    update.finalize();

    // Need to manually update the update before cache refresh,
    // and before broadcast with add_update.
    // The Err case of to_document should be unreachable,
    // but will fail to update cache in that case.
    if let Ok(update_doc) = to_document(&update) {
      let _ = update_one_by_id(
        &db_client().await.updates,
        &update.id,
        mungos::update::Update::Set(update_doc),
        None,
      )
      .await;
      refresh_procedure_state_cache().await;
    }

    update_update(update.clone()).await?;

    Ok(update)
  })
}
