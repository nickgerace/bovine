use crate::{consts::platform::NEWLINE, docker, types::cli::List};
use anyhow::Result;

pub async fn list(opt: &List, docker_socket_path: Option<String>) -> Result<()> {
    let docker = docker::docker_client(docker_socket_path).await?;
    let list = docker::list(&docker, opt.short, opt.running).await?;
    if !list.is_empty() {
        println!("{}", list.join(NEWLINE));
    }
    Ok(())
}
