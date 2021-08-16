use crate::{consts::platform::NEWLINE, docker, error::Error, rancher, types::cli::Get};
use anyhow::Result;

pub async fn get(opt: &Get, docker_socket_path: Option<String>) -> Result<()> {
    let docker = docker::docker_client(docker_socket_path).await?;
    let inspection = docker.inspect_container(&opt.container_id, None).await?;

    match &inspection.state {
        Some(s) => {
            println!(
                "{},{}{}",
                serde_json::to_string_pretty(&rancher::build_config(
                    match inspection.config {
                        Some(s) => s,
                        None =>
                            return Err(Error::DockerContainerConfigNotFound(
                                opt.container_id.clone()
                            )
                            .into()),
                    },
                    match inspection.host_config {
                        Some(s) => s,
                        None =>
                            return Err(Error::DockerContainerConfigNotFound(
                                opt.container_id.clone()
                            )
                            .into()),
                    },
                    None
                ))?,
                NEWLINE,
                serde_json::to_string_pretty(s)?
            );
            Ok(())
        }
        None => Err(Error::DockerContainerStateNotFound(opt.container_id.clone()).into()),
    }
}
