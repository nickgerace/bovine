use crate::error::Error;
use serde::Serialize;

#[derive(Serialize)]
pub struct Version {
    pub bovine: BovineVersion,
    pub docker: DockerVersion,
}

#[derive(Serialize)]
pub struct BovineVersion {
    pub version: String,
    #[serde(rename(serialize = "os/arch"))]
    pub os_arch: String,
}

#[derive(Serialize)]
pub struct DockerVersion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename(serialize = "os/arch"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_kernel_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_socket_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub fn new_docker_version_with_connection_failure() -> DockerVersion {
    DockerVersion {
        version: None,
        os_arch: None,
        api_version: None,
        linux_kernel_version: None,
        git_commit: None,
        docker_socket_path: None,
        error: Some(Error::DockerSocketConnectFailure.to_string()),
    }
}
