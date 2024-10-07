use std::{collections::HashMap, path::Path};

use anyhow::Context;
use command::run_komodo_command;
use formatting::format_serror;
use komodo_client::entities::{
  update::Log, CloneArgs, EnvironmentVar,
};

use crate::{get_commit_hash_log, GitRes};

/// This will pull in a way that handles edge cases
/// from possible state of the repo. For example, the user
/// can change branch after clone, or even the remote.
#[tracing::instrument(
  level = "debug",
  skip(
    clone_args,
    access_token,
    environment,
    secrets,
    core_replacers
  )
)]
#[allow(clippy::too_many_arguments)]
pub async fn pull<T>(
  clone_args: T,
  repo_dir: &Path,
  access_token: Option<String>,
  environment: &[EnvironmentVar],
  env_file_path: &str,
  // if skip_secret_interp is none, make sure to pass None here
  secrets: Option<&HashMap<String, String>>,
  core_replacers: &[(String, String)],
) -> anyhow::Result<GitRes>
where
  T: Into<CloneArgs> + std::fmt::Debug,
{
  let args: CloneArgs = clone_args.into();
  let path = args.path(repo_dir);
  let path_display = path.display();
  let repo_url =
    args.remote_url(access_token.as_ref().map(String::as_str))?;

  // Set remote url
  let mut set_remote = run_komodo_command(
    "set git remote",
    format!(
      "cd {path_display} && git remote set-url origin {repo_url}"
    ),
  )
  .await;

  if !set_remote.success {
    if let Some(token) = access_token {
      set_remote.command =
        set_remote.command.replace(&token, "<TOKEN>");
      set_remote.stdout =
        set_remote.stdout.replace(&token, "<TOKEN>");
      set_remote.stderr =
        set_remote.stderr.replace(&token, "<TOKEN>");
    }
    return Ok(GitRes {
      logs: vec![set_remote],
      hash: None,
      message: None,
      env_file_path: None,
    });
  }

  let checkout = run_komodo_command(
    "checkout branch",
    format!("cd {path_display} && git checkout -f {}", args.branch),
  )
  .await;

  if !checkout.success {
    return Ok(GitRes {
      logs: vec![checkout],
      hash: None,
      message: None,
      env_file_path: None,
    });
  }

  let command = format!(
    "cd {path_display} && git pull --rebase --force origin {}",
    args.branch
  );

  let pull_log = run_komodo_command("git pull", command).await;

  let mut logs = vec![pull_log];

  if !logs[0].success {
    return Ok(GitRes {
      logs,
      hash: None,
      message: None,
      env_file_path: None,
    });
  }

  if let Some(commit) = args.commit {
    let reset_log = run_komodo_command(
      "set commit",
      format!("cd {path_display} && git reset --hard {commit}"),
    )
    .await;
    logs.push(reset_log);
  }

  let (hash, message) = match get_commit_hash_log(&path).await {
    Ok((log, hash, message)) => {
      logs.push(log);
      (Some(hash), Some(message))
    }
    Err(e) => {
      logs.push(Log::simple(
        "latest commit",
        format_serror(
          &e.context("failed to get latest commit").into(),
        ),
      ));
      (None, None)
    }
  };

  let Ok(env_file_path) = crate::environment::write_file(
    environment,
    env_file_path,
    secrets,
    &path,
    &mut logs,
  )
  .await
  else {
    return Ok(GitRes {
      logs,
      hash,
      message,
      env_file_path: None,
    });
  };

  if let Some(command) = args.on_pull {
    if !command.path.is_empty() && !command.command.is_empty() {
      let on_pull_path = path.join(&command.path);
      if let Some(secrets) = secrets {
        let (full_command, mut replacers) =
          match svi::interpolate_variables(
            &command.command,
            secrets,
            svi::Interpolator::DoubleBrackets,
            true,
          )
          .context(
            "failed to interpolate secrets into on_pull command",
          ) {
            Ok(res) => res,
            Err(e) => {
              logs.push(Log::error(
                "interpolate secrets - on_pull",
                format_serror(&e.into()),
              ));
              return Ok(GitRes {
                logs,
                hash,
                message,
                env_file_path: None,
              });
            }
          };
        replacers.extend(core_replacers.to_owned());
        let mut on_pull_log = run_komodo_command(
          "on pull",
          format!("cd {} && {full_command}", on_pull_path.display()),
        )
        .await;

        on_pull_log.command =
          svi::replace_in_string(&on_pull_log.command, &replacers);
        on_pull_log.stdout =
          svi::replace_in_string(&on_pull_log.stdout, &replacers);
        on_pull_log.stderr =
          svi::replace_in_string(&on_pull_log.stderr, &replacers);

        tracing::debug!(
          "run repo on_pull command | command: {} | cwd: {:?}",
          on_pull_log.command,
          on_pull_path
        );

        logs.push(on_pull_log);
      } else {
        let on_pull_log = run_komodo_command(
          "on pull",
          format!(
            "cd {} && {}",
            on_pull_path.display(),
            command.command
          ),
        )
        .await;
        tracing::debug!(
          "run repo on_pull command | command: {} | cwd: {:?}",
          command.command,
          on_pull_path
        );
        logs.push(on_pull_log);
      }
    }
  }

  Ok(GitRes {
    logs,
    hash,
    message,
    env_file_path,
  })
}