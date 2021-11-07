use crate::{docker, error::Error, rancher, types::cli::Restart, util};
use anyhow::Result;
use log::info;

pub async fn restart(opt: &Restart, docker_socket_path: Option<String>) -> Result<()> {
    let docker = docker::docker_client(docker_socket_path).await?;
    match rancher::is_bovine(&docker.inspect_container(&opt.container_id, None).await?)? {
        true => {
            info!("Restarting Rancher container: {}", &opt.container_id);
            match docker.restart_container(&opt.container_id, None).await {
                Ok(_) => Ok(()),
                Err(e) => {
                    util::log_bollard_error(&e);
                    Err(Error::DockerContainerRestartFailure.into())
                }
            }
        }
        false => Err(Error::NotBovineContainer.into()),
    }
}
