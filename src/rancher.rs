use crate::{
    consts::package::{LABEL, LABEL_VALUE, UNKNOWN},
    error::Error,
    util,
};
use anyhow::Result;
use bollard::{
    container::Config,
    models::{
        ContainerConfig, ContainerInspectResponse, HostConfig, RestartPolicy, RestartPolicyNameEnum,
    },
    Docker,
};
use std::collections::HashMap;

pub async fn launch_rancher(docker: Docker, config: Config<String>) -> Result<()> {
    let id = docker
        .create_container::<String, String>(None, config)
        .await?
        .id;
    docker.start_container::<String>(&id, None).await?;
    println!(
        "Rancher container is running: {}",
        util::get_first_n_chars(id, 12)
    );
    Ok(())
}

pub fn build_bind(provided: &str, internal: &str) -> Result<String> {
    Ok(util::join_by_colon(
        &util::canonicalize_str(provided)?,
        internal,
    ))
}

pub fn build_config(
    config: ContainerConfig,
    host_config: HostConfig,
    volumes_from: Option<Vec<String>>,
) -> Config<String> {
    Config::<String> {
        image: config.image.clone(),
        env: config.env,
        cmd: config.cmd,
        exposed_ports: config.exposed_ports,
        host_config: Some(HostConfig {
            binds: host_config.binds,
            privileged: Some(true),
            port_bindings: host_config.port_bindings,
            restart_policy: Some(RestartPolicy {
                name: Some(RestartPolicyNameEnum::UNLESS_STOPPED),
                maximum_retry_count: None,
            }),
            volumes_from,
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub fn get_labels() -> HashMap<String, String> {
    let mut labels = HashMap::new();
    labels.insert(LABEL.to_string(), LABEL_VALUE.to_string());
    labels
}

pub fn is_bovine(inspection: &ContainerInspectResponse) -> Result<bool, Error> {
    let id = match &inspection.id {
        Some(s) => s,
        None => UNKNOWN,
    };
    match &inspection.config {
        Some(s) => match &s.labels {
            Some(labels) => Ok(labels.contains_key(LABEL)),
            None => Err(Error::DockerContainerLabelsNotFound(id.to_string())),
        },
        None => Err(Error::DockerContainerConfigNotFound(id.to_string())),
    }
}
