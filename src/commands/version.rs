use crate::{
    consts::{
        package::{NAME, VERSION},
        platform::NEWLINE,
    },
    docker,
    error::Error,
    types::cli::Version,
    util,
};
use std::env::consts;

pub async fn version(opt: &Version, docker_socket_path: Option<String>) {
    match opt.short {
        true => println!("{}", VERSION),
        false => full_version(docker_socket_path).await,
    }
}

async fn full_version(docker_socket_path: Option<String>) {
    let mut to_print = vec![
        format!("{}:", NAME),
        format!("  version: {}", VERSION),
        format!("  os/arch: {}/{}", consts::OS, consts::ARCH),
        "docker:".to_string(),
    ];
    match docker::docker_client(docker_socket_path.clone()).await {
        Ok(docker) => match docker.version().await {
            Ok(version) => {
                if let Some(s) = version.version {
                    to_print.push(format!("  version: {}", s))
                }
                if let (Some(os), Some(arch)) = (version.os, version.arch) {
                    to_print.push(format!("  os/arch: {}/{}", os, arch));
                }
                if let Some(s) = version.api_version {
                    to_print.push(format!("  api version: {}", s));
                }
                if let Some(s) = version.kernel_version {
                    to_print.push(format!("  linux kernel version: {}", s))
                }
                if let Some(s) = version.git_commit {
                    to_print.push(format!("  git commit: {}", s))
                }
                if let Some(s) = docker_socket_path {
                    to_print.push(format!("  socket path: {}", s))
                }
            }
            Err(e) => {
                util::log_bollard_error(&e);
                to_print.push(format!("  {}", Error::DockerSocketConnectFailure));
            }
        },
        Err(e) => to_print.push(format!("  {}", e)),
    };
    println!("{}", to_print.join(NEWLINE));
}
