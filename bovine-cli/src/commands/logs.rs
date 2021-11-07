use crate::{docker, types::cli::Logs};
use anyhow::Result;
use bollard::container::LogsOptions;
use futures::StreamExt;

// We do not want to log a formatted log, so we print directly to STDOUT and STDERR as needed.
// In addition, we want to send stream errors to STDERR, and _all_ given logs to STDOUT.
// This pattern would be difficult to make user friendly (and maintaible) if implemented with logging.
pub async fn logs(opt: &Logs, docker_socket_path: Option<String>) -> Result<()> {
    let docker = docker::docker_client(docker_socket_path).await?;
    let id = docker::get_container_id_greedy(&docker, &opt.container_id).await?;

    let mut stream = docker.logs::<String>(
        &id,
        Some(LogsOptions {
            follow: opt.follow,
            stdout: true,
            stderr: true,
            ..Default::default()
        }),
    );
    while let Some(msg) = stream.next().await {
        match msg {
            Ok(o) => print!("{}", o),
            Err(e) => eprint!("{}", e),
        }
    }
    Ok(())
}
