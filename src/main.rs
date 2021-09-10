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
    if opt.debug {
        env::set_var("RUST_LOG", "debug");
        env_logger::init();
    } else if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
        // Since we are using `INFO` logs for user messages, we want to print to STDOUT and not STDERR.
        env_logger::builder()
            .format(|buf, record| writeln!(buf, "{}", record.args()))
            .target(env_logger::Target::Stdout)
            .init();
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
