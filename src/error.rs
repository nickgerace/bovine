use bollard::container::LogOutput;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("could not create docker client (check if docker is running)")]
    DockerClientCreateFailure,
    #[error("could not connect to docker (check if docker is running)")]
    DockerSocketConnectFailure,
    #[error("call to docker socket failed (check if docker is running)")]
    DockerRuntimeFailure,
    #[error("container does not exist")]
    DockerContainerDoesNotExist,
    #[error(
        "restart error occurred (perhaps there is already a container using the same port(s))"
    )]
    DockerContainerRestartFailure,

    #[error("could not find config for inspected container: {0}")]
    DockerContainerConfigNotFound(String),
    #[error("could not find image in container config: {0}")]
    DockerContainerImageNotFound(String),
    #[error("container state not found for container: {0}")]
    DockerContainerStateNotFound(String),
    #[error("labels not found for container: {0}")]
    DockerContainerLabelsNotFound(String),

    #[error("invalid tag found: {0}")]
    OCIImageTagInvalid(String),
    #[error("splitting OCI image name into its short name and tag failed: {0}")]
    OCIImageSplitFailure(String),
    #[error("could not find bootstrap password within the line: {0}")]
    LogMessageScrapingFailure(LogOutput),

    #[error("container not managed by Bovine")]
    NotBovineContainer,
    #[error("container ID must be provided or \"--all\" flag must be set")]
    StopOrDeleteContainerNotProvided,
    #[error("container ID must be provided or at least one running Rancher container must exist")]
    LogsContainerNotProvided,
}
