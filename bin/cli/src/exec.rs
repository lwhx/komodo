use std::time::Duration;

use colored::Colorize;
use komodo_client::{
  api::execute::{BatchExecutionResponse, Execution},
  entities::update::Update,
};

use crate::{
  helpers::wait_for_enter,
  state::{cli_args, komodo_client},
};

pub enum ExecutionResult {
  Single(Box<Update>),
  Batch(BatchExecutionResponse),
}

pub async fn run(execution: Execution) -> anyhow::Result<()> {
  if matches!(execution, Execution::None(_)) {
    println!("Got 'none' execution. Doing nothing...");
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("Finished doing nothing. Exiting...");
    std::process::exit(0);
  }

  println!("\n{}: Execution", "Mode".dimmed());
  match &execution {
    Execution::None(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RunAction(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::BatchRunAction(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RunProcedure(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::BatchRunProcedure(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RunBuild(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::BatchRunBuild(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::CancelBuild(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::Deploy(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::BatchDeploy(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PullDeployment(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StartDeployment(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RestartDeployment(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PauseDeployment(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::UnpauseDeployment(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StopDeployment(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::DestroyDeployment(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::BatchDestroyDeployment(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::CloneRepo(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::BatchCloneRepo(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PullRepo(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::BatchPullRepo(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::BuildRepo(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::BatchBuildRepo(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::CancelRepoBuild(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StartContainer(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RestartContainer(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PauseContainer(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::UnpauseContainer(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StopContainer(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::DestroyContainer(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StartAllContainers(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RestartAllContainers(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PauseAllContainers(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::UnpauseAllContainers(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StopAllContainers(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneContainers(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::DeleteNetwork(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneNetworks(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::DeleteImage(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneImages(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::DeleteVolume(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneVolumes(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneDockerBuilders(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneBuildx(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneSystem(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RunSync(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::CommitSync(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::DeployStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::BatchDeployStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::DeployStackIfChanged(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::BatchDeployStackIfChanged(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PullStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::BatchPullStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StartStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RestartStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PauseStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::UnpauseStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StopStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::DestroyStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::BatchDestroyStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::TestAlerter(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::Sleep(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
  }

  if !cli_args().yes {
    wait_for_enter("run execution")?;
  }

  info!("Running Execution...");

  let res = match execution {
    Execution::RunAction(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::BatchRunAction(request) => komodo_client()
      .execute(request)
      .await
      .map(ExecutionResult::Batch),
    Execution::RunProcedure(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::BatchRunProcedure(request) => komodo_client()
      .execute(request)
      .await
      .map(ExecutionResult::Batch),
    Execution::RunBuild(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::BatchRunBuild(request) => komodo_client()
      .execute(request)
      .await
      .map(ExecutionResult::Batch),
    Execution::CancelBuild(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::Deploy(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::BatchDeploy(request) => komodo_client()
      .execute(request)
      .await
      .map(ExecutionResult::Batch),
    Execution::PullDeployment(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::StartDeployment(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::RestartDeployment(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::PauseDeployment(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::UnpauseDeployment(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::StopDeployment(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::DestroyDeployment(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::BatchDestroyDeployment(request) => komodo_client()
      .execute(request)
      .await
      .map(ExecutionResult::Batch),
    Execution::CloneRepo(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::BatchCloneRepo(request) => komodo_client()
      .execute(request)
      .await
      .map(ExecutionResult::Batch),
    Execution::PullRepo(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::BatchPullRepo(request) => komodo_client()
      .execute(request)
      .await
      .map(ExecutionResult::Batch),
    Execution::BuildRepo(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::BatchBuildRepo(request) => komodo_client()
      .execute(request)
      .await
      .map(ExecutionResult::Batch),
    Execution::CancelRepoBuild(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::StartContainer(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::RestartContainer(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::PauseContainer(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::UnpauseContainer(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::StopContainer(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::DestroyContainer(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::StartAllContainers(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::RestartAllContainers(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::PauseAllContainers(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::UnpauseAllContainers(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::StopAllContainers(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::PruneContainers(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::DeleteNetwork(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::PruneNetworks(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::DeleteImage(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::PruneImages(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::DeleteVolume(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::PruneVolumes(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::PruneDockerBuilders(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::PruneBuildx(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::PruneSystem(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::RunSync(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::CommitSync(request) => komodo_client()
      .write(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::DeployStack(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::BatchDeployStack(request) => komodo_client()
      .execute(request)
      .await
      .map(ExecutionResult::Batch),
    Execution::DeployStackIfChanged(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::BatchDeployStackIfChanged(request) => komodo_client()
      .execute(request)
      .await
      .map(ExecutionResult::Batch),
    Execution::PullStack(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::BatchPullStack(request) => komodo_client()
      .execute(request)
      .await
      .map(ExecutionResult::Batch),
    Execution::StartStack(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::RestartStack(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::PauseStack(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::UnpauseStack(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::StopStack(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::DestroyStack(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::BatchDestroyStack(request) => komodo_client()
      .execute(request)
      .await
      .map(ExecutionResult::Batch),
    Execution::TestAlerter(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u.into())),
    Execution::Sleep(request) => {
      let duration =
        Duration::from_millis(request.duration_ms as u64);
      tokio::time::sleep(duration).await;
      println!("Finished sleeping!");
      std::process::exit(0)
    }
    Execution::None(_) => unreachable!(),
  };

  match res {
    Ok(ExecutionResult::Single(update)) => {
      println!("\n{}: {update:#?}", "SUCCESS".green())
    }
    Ok(ExecutionResult::Batch(update)) => {
      println!("\n{}: {update:#?}", "SUCCESS".green())
    }
    Err(e) => println!("{}\n\n{e:#?}", "ERROR".red()),
  }

  Ok(())
}
