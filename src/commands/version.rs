use crate::{
    consts::package::VERSION,
    docker,
    types::{cli, version},
    util,
};
use anyhow::Result;
use log::{error, info};
use serde_json;
use std::env::consts;

pub async fn version(opt: &cli::Version, docker_socket_path: Option<String>) -> Result<()> {
    match opt.short {
        true => {
            info!("{}", VERSION);
            Ok(())
        }
        false => full_version(docker_socket_path).await,
    }
}

async fn full_version(docker_socket_path: Option<String>) -> Result<()> {
    info!(
        "{}",
        serde_json::to_string_pretty(&version::Version {
            bovine: version::BovineVersion {
                version: VERSION.to_string(),
                os_arch: format!("{}/{}", consts::OS, consts::ARCH),
            },
            docker: match docker::docker_client(docker_socket_path.clone()).await {
                Ok(docker) => match docker.version().await {
                    Ok(version) => version::DockerVersion {
                        version: version.version,
                        os_arch: match (version.os, version.arch) {
                            (Some(os), Some(arch)) => Some(format!("{}/{}", os, arch)),
                            _ => None,
                        },
                        api_version: version.api_version,
                        linux_kernel_version: version.kernel_version,
                        git_commit: version.git_commit,
                        docker_socket_path,
                        error: None,
                    },
                    Err(e) => {
                        util::log_bollard_error(&e);
                        version::new_docker_version_with_connection_failure()
                    }
                },
                Err(e) => {
                    error!("{}", e);
                    version::new_docker_version_with_connection_failure()
                }
            },
        })?
    );
    Ok(())
}
