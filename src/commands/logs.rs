use crate::{docker, error::Error, types::cli::Logs};
use anyhow::Result;
use bollard::container::LogsOptions;
use futures::StreamExt;

pub async fn logs(opt: &Logs, docker_socket_path: Option<String>) -> Result<()> {
    let docker = docker::docker_client(docker_socket_path).await?;

    // We set the ID upfront since list items are borrowed. We will need to clone them.
    let id = match &opt.container_id {
        Some(s) => s.to_string(),
        None => {
            let list = docker::list(&docker, true, true).await?;
            match list.len() {
                0 => return Err(Error::LogsContainerNotProvided.into()),
                _ => list[0].clone(),
            }
        }
    };

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
