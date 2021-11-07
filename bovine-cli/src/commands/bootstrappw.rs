use crate::{consts, docker, error::Error, types::cli::BootstrapPW};
use anyhow::Result;
use bollard::container::LogsOptions;
use futures::StreamExt;

// We do not want to log a formatted log, so we print directly to STDOUT and STDERR as needed.
// In addition, we want to send stream errors to STDERR, and _all_ given logs to STDOUT.
// This pattern would be difficult to make user friendly (and maintaible) if implemented with logging.
pub async fn bootstrappw(opt: &BootstrapPW, docker_socket_path: Option<String>) -> Result<()> {
    let docker = docker::docker_client(docker_socket_path).await?;
    let id = docker::get_container_id_greedy(&docker, &opt.container_id).await?;

    let mut stream = docker.logs::<String>(
        &id,
        Some(LogsOptions {
            // If we are looking for the boostrap password and would like to wait, we need to
            // set follow to true.
            follow: opt.wait,
            stdout: true,
            stderr: true,
            ..Default::default()
        }),
    );
    while let Some(msg) = stream.next().await {
        match msg {
            Ok(o)
                if o.to_string()
                    .contains(consts::rancher::BOOTSTRAP_PASSWORD_SEARCH_TERM) =>
            {
                match opt.verbose {
                    // We convert the log message to a "String" twice since this match guard
                    // will only pass once. This double computation can only occur once due
                    // to the "break" statement.
                    true => print!("{}", o.to_string()),
                    false => match o
                        .to_string()
                        .rsplit(consts::rancher::BOOTSTRAP_PASSWORD_SEARCH_TERM)
                        .next()
                    {
                        Some(s) => print!("{}", s),
                        None => return Err(Error::BootstrapPasswordScrapingFailure(o).into()),
                    },
                }
                return Ok(());
            }
            Ok(_) => {}
            Err(e) => eprint!("{}", e),
        }
    }
    Err(Error::BootstrapPasswordNotFound.into())
}
