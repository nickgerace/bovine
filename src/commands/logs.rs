use crate::{consts, docker, error::Error, types::cli::Logs};
use anyhow::Result;
use bollard::container::LogsOptions;
use futures::StreamExt;

pub async fn logs(opt: &Logs, docker_socket_path: Option<String>) -> Result<()> {
    let docker = docker::docker_client(docker_socket_path).await?;

    // We set the ID upfront since list items are borrowed. We will need to clone them.
    let id = match &opt.container_id {
        Some(s) => s.to_string(),
        None => {
            let list = docker::list(&docker, true, true).await?;
            match list.len() {
                0 => return Err(Error::LogsContainerNotProvided.into()),
                _ => list[0].clone(),
            }
        }
    };

    // Only use the follow value if we are not looking for the bootstrap password.
    let mut stream = docker.logs::<String>(
        &id,
        Some(LogsOptions {
            follow: match opt.find_bootstrap_password {
                true => false,
                false => opt.follow,
            },
            stdout: true,
            stderr: true,
            ..Default::default()
        }),
    );

    // We use a match statement with repeated code here because the alternative would be to check
    // if we are looking for the bootstrap password at every iteration of the stream. We only want
    // to perform the check once for optimal performance.
    match opt.find_bootstrap_password {
        true => {
            while let Some(msg) = stream.next().await {
                match msg {
                    Ok(o)
                        if o.to_string()
                            .contains(consts::rancher::BOOTSTRAP_PASSWORD_SEARCH) =>
                    {
                        match opt.verbose {
                            // We convert the log message to a "String" twice since this match guard
                            // will only pass once. This double computation can only occur once due
                            // to the "break" statement.
                            true => print!("{}", o.to_string()),
                            false => match o
                                .to_string()
                                .rsplit(consts::rancher::BOOTSTRAP_PASSWORD_SEARCH)
                                .next()
                            {
                                Some(s) => print!("{}", s),
                                None => {
                                    return Err(Error::BootstrapPasswordScrapingFailure(o).into())
                                }
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
        false => {
            while let Some(msg) = stream.next().await {
                match msg {
                    Ok(o) => print!("{}", o),
                    Err(e) => eprint!("{}", e),
                }
            }
            Ok(())
        }
    }
}
