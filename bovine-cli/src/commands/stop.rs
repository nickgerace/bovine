use crate::{docker, error::Error, types::cli::Stop};
use anyhow::Result;
use log::info;

pub async fn stop(opt: &Stop, docker_socket_path: Option<String>) -> Result<()> {
    let docker = docker::docker_client(docker_socket_path).await?;
    match &opt.container_id {
        Some(id) => match opt.dry_run {
            true => {
                info!("{}", &id);
                Ok(())
            }
            false => docker::stop_container(&docker, id, opt.delete).await,
        },
        None if opt.all => {
            // We pass in the "!opt.delete" flag to check if we only want to list running
            // containers because we do not need to stop containers that are not running. However,
            // we want to *delete* containers that are not running. Thus, if the user passes in the
            // delete flag, we should delete running *and* non-running containers. If the user does
            // not pass in the delete flag, we should stop running containers and avoid processing
            // non-running containers.
            for list_item in docker::list(&docker, true, !opt.delete).await? {
                match opt.dry_run {
                    true => info!("{}", &list_item),
                    false => docker::stop_container(&docker, &list_item, opt.delete).await?,
                }
            }
            Ok(())
        }
        None => Err(Error::StopOrDeleteContainerNotProvided.into()),
    }
}
