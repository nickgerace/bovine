use crate::{
    consts::package::{LABEL, UNKNOWN},
    error::Error,
    rancher, util,
};
use anyhow::Result;
use bollard::{
    container::ListContainersOptions,
    errors::Error::{DockerResponseNotFoundError, DockerResponseNotModifiedError},
    image::{CreateImageOptions, ListImagesOptions},
    models::{ContainerConfig, ContainerStateStatusEnum::RUNNING, HostConfig},
    Docker, API_DEFAULT_VERSION,
};
use futures::TryStreamExt;
use log::{debug, error, info};
use std::collections::HashMap;

pub async fn docker_client(socket_path: Option<String>) -> Result<Docker, Error> {
    match socket_path {
        Some(s) => match Docker::connect_with_socket(&s, 120, API_DEFAULT_VERSION) {
            Ok(o) => Ok(o),
            Err(e) => {
                util::log_bollard_error(&e);
                Err(Error::DockerClientCreateFailure)
            }
        },
        None => match Docker::connect_with_socket_defaults() {
            Ok(o) => Ok(o),
            Err(e) => {
                util::log_bollard_error(&e);
                Err(Error::DockerClientCreateFailure)
            }
        },
    }
}

pub async fn get_configs(
    docker: &Docker,
    container_id: &str,
) -> Result<(ContainerConfig, HostConfig)> {
    let inspection = docker.inspect_container(container_id, None).await?;

    Ok((
        match inspection.config {
            Some(s) => s,
            None => {
                return Err(Error::DockerContainerConfigNotFound(container_id.to_string()).into())
            }
        },
        match inspection.host_config {
            Some(s) => s,
            None => {
                return Err(Error::DockerContainerConfigNotFound(container_id.to_string()).into())
            }
        },
    ))
}

pub async fn stop_container(docker: &Docker, container_id: &str, delete: bool) -> Result<()> {
    let inspection = docker.inspect_container(container_id, None).await?;
    match rancher::is_bovine(&inspection)? {
        true => {
            debug!("processing container for stop: {}", container_id);
            match docker.stop_container(container_id, None).await {
                Ok(_) => info!("Stopped Rancher container: {}", container_id),
                Err(e) => match e {
                    DockerResponseNotModifiedError { message: _ } => {
                        info!(
                            "Container not modified (may have already been stopped): {}",
                            container_id
                        )
                    }
                    DockerResponseNotFoundError { message: _ } => {
                        return Err(Error::DockerContainerDoesNotExist.into())
                    }
                    _ => return Err(e.into()),
                },
            }
            if delete {
                debug!("processing container for delete: {}", container_id);
                match docker.remove_container(container_id, None).await {
                    Ok(_) => info!("Deleted Rancher container: {}", container_id),
                    Err(e) => match e {
                        DockerResponseNotFoundError { message: _ } => {
                            info!(
                                "Container not found (may have already been deleted): {}",
                                container_id
                            )
                        }
                        _ => return Err(e.into()),
                    },
                }
                if let Some(mounts) = inspection.mounts {
                    for mount_point in mounts {
                        match mount_point.name {
                            Some(s) => match docker.remove_volume(&s, None).await {
                                Ok(_) => {
                                    debug!("deleted volume ({}) for container: {}", s, container_id)
                                }
                                Err(e) => error!("{}", e),
                            },
                            None => error!(
                                "Name not found for a mount point for container {}",
                                container_id
                            ),
                        }
                    }
                }
                info!("Deleted volumes for container: {}", container_id);
            }
            Ok(())
        }
        false => Err(Error::NotBovineContainer.into()),
    }
}

pub async fn get_container_id_greedy(
    docker: &Docker,
    container_id: &Option<String>,
) -> Result<String> {
    match container_id {
        Some(s) => Ok(s.to_string()),
        None => {
            let containers = list(docker, true, true).await?;
            match containers.len() {
                0 => Err(Error::LogsContainerNotProvided.into()),
                _ => Ok(containers[0].clone()),
            }
        }
    }
}

pub async fn pull_image(docker: &Docker, image: &str, force_pull: bool) -> Result<(), Error> {
    async fn pull_image_helper(docker: &Docker, image: &str) -> Result<(), Error> {
        match docker
            .create_image(
                Some(CreateImageOptions {
                    from_image: image,
                    ..Default::default()
                }),
                None,
                None,
            )
            .try_collect::<Vec<_>>()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                util::log_bollard_error(&e);
                Err(Error::DockerRuntimeFailure)
            }
        }
    }

    let mut filters = HashMap::with_capacity(1);
    filters.insert("reference", vec![image]);
    match docker
        .list_images::<&str>(Some(ListImagesOptions {
            filters,
            ..Default::default()
        }))
        .await
    {
        Ok(o) => match o.is_empty() {
            true => {
                info!("Pulling [{}], this may take awhile...", image);
                pull_image_helper(docker, image).await
            }
            false if force_pull => {
                info!("Force pulling [{}], this may take awhile...", image);
                pull_image_helper(docker, image).await
            }
            false => {
                info!("Image found locally: [{}]", image);
                Ok(())
            }
        },
        Err(e) => {
            util::log_bollard_error(&e);
            Err(Error::DockerRuntimeFailure)
        }
    }
}

pub async fn list(docker: &Docker, short: bool, running: bool) -> Result<Vec<String>> {
    fn log_list_error(item: &str) {
        error!(
            "could not find {} for container in list (filtered with label={})",
            item, LABEL
        )
    }

    let mut filters: HashMap<String, Vec<String>> = HashMap::new();
    filters.insert("label".to_string(), vec![LABEL.to_string()]);

    let mut list = Vec::new();
    for container in docker
        .list_containers(Some(ListContainersOptions {
            all: true,
            filters,
            ..Default::default()
        }))
        .await?
    {
        let state = match container.state {
            Some(s) => s,
            None => {
                log_list_error("state");
                UNKNOWN.to_string()
            }
        };
        if running && state != RUNNING.to_string() {
            continue;
        }

        match container.id {
            Some(s) => {
                let id = util::get_first_n_chars(s, 12);
                match short {
                    true => list.push(id),
                    false => list.push(format!(
                        "{} [{}] ({}) > {}",
                        id,
                        match container.image {
                            Some(s) => s,
                            None => UNKNOWN.to_string(),
                        },
                        state,
                        match container.status {
                            Some(s) => s,
                            None => UNKNOWN.to_string(),
                        }
                    )),
                }
            }
            None => log_list_error("ID"),
        }
    }
    debug!("containers found for list: {:?}", list);
    Ok(list)
}
