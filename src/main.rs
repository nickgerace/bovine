use crate::error::Error;
use anyhow::Result;
use clap::Clap;
use std::{env, io::Write};
use types::cli::{Opt, SubCommand};

mod commands;
mod consts;
mod docker;
mod error;
mod rancher;
mod types;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::parse();

    // This match statement covers `RUST_LOG` inputs and sets the logger accordingly.
    //
    // If debug flag has been set:
    // - override `RUST_LOG` regardless of its value --> standard mode
    //
    // Else:
    // - user has not set `RUST_LOG` --> set to `info` --> user mode
    // - user has disabled `RUST_LOG` --> set to `info` --> user mode
    // - user has already set `RUST_LOG` to `info` --> no level change --> user mode
    // - user has set `RUST_LOG` above `info` --> no level change --> standard mode
    match opt.debug {
        true => start_env_logger(false, Some("debug")),
        false => match env::var("RUST_LOG") {
            Err(_) => start_env_logger(true, Some("info")),
            Ok(o) if o == "off" => start_env_logger(true, Some("info")),
            Ok(o) if o == "info" => start_env_logger(true, None),
            Ok(_) => start_env_logger(false, None),
        },
    }

    match opt.subcmd {
        SubCommand::Get(o) => commands::get::get(&o, opt.docker_socket_path).await?,
        SubCommand::List(o) => commands::list::list(&o, opt.docker_socket_path).await?,
        SubCommand::Logs(_) if opt.debug => return Err(Error::LogsDebugModeEnabled.into()),
        SubCommand::Logs(o) => commands::logs::logs(&o, opt.docker_socket_path).await?,
        SubCommand::Restart(o) => commands::restart::restart(&o, opt.docker_socket_path).await?,
        SubCommand::Run(o) => commands::run::run(&o, opt.docker_socket_path).await?,
        SubCommand::Stop(o) => commands::stop::stop(&o, opt.docker_socket_path).await?,
        SubCommand::Upgrade(o) => commands::upgrade::upgrade(&o, opt.docker_socket_path).await?,
        SubCommand::Version(o) => commands::version::version(&o, opt.docker_socket_path).await?,
    }
    Ok(())
}

fn start_env_logger(user_mode: bool, set_level: Option<&str>) {
    if let Some(s) = set_level {
        env::set_var("RUST_LOG", s);
    }
    match user_mode {
        true => {
            // Since we are using `INFO` logs for user messages, we want to print to STDOUT and not STDERR.
            // In this mode, we want the `INFO` logs to perform similarly to standard print statements.
            env_logger::builder()
                .format(|buf, record| writeln!(buf, "{}", record.args()))
                .target(env_logger::Target::Stdout)
                .init()
        }
        false => env_logger::init(),
    }
}
