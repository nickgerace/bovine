use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(
    name = "bovine",
    about = "Manager for single node Rancher clusters: https://github.com/nickgerace/bovine",
    global_setting = AppSettings::ColorNever,
    global_setting = AppSettings::DisableVersionFlag,
)]
pub struct Opt {
    #[clap(
        long,
        short,
        about = "Display debug-level logs (sets \"RUST_LOG\" to debug)"
    )]
    pub debug: bool,
    #[clap(long, short = 's', about = "Path to Docker socket")]
    pub docker_socket_path: Option<String>,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(
        name = "get",
        about = "Get and display information for a given Rancher container (JSON)"
    )]
    Get(Get),
    #[clap(
        name = "list",
        about = "List all running and non-running Rancher clusters from Bovine"
    )]
    List(List),
    #[clap(
        name = "logs",
        about = "Print logs for a Rancher container managed by Bovine"
    )]
    Logs(Logs),
    #[clap(
        name = "restart",
        about = "Restart a Rancher container or start a stopped Rancher container"
    )]
    Restart(Restart),
    #[clap(name = "run", about = "Create and start a new Rancher container")]
    Run(Box<Run>),
    #[clap(
        name = "stop",
        about = "Stop a Rancher container (and optionally delete it)"
    )]
    Stop(Stop),
    #[clap(
        name = "upgrade",
        about = "Upgrade an existing Rancher container to a new tag"
    )]
    Upgrade(Box<Upgrade>),
    #[clap(name = "version", about = "Display extensive version information")]
    Version(Version),
}

#[derive(Clap, Debug)]
pub struct Get {
    #[clap(about = "ID of the Rancher container running")]
    pub container_id: String,
}

#[derive(Clap, Debug)]
pub struct List {
    #[clap(long, short, about = "Show only containing that are running")]
    pub running: bool,
    #[clap(long, short, about = "Show container IDs only")]
    pub short: bool,
}

#[derive(Clap, Debug)]
pub struct Logs {
    #[clap(
        about = "ID of the Rancher container running or not-running (if empty, will select a Rancher container at random)"
    )]
    pub container_id: Option<String>,
    #[clap(long, short, about = "Follow logs")]
    pub follow: bool,
}

#[derive(Clap, Debug)]
pub struct Restart {
    #[clap(about = "ID of the Rancher container running")]
    pub container_id: String,
}

#[derive(Clap, Debug)]
pub struct Run {
    #[clap(long, about = "Display the Docker API container config (JSON)")]
    pub dry_run: bool,
    #[clap(
        long,
        short,
        about = "Specify image for Rancher",
        default_value = "rancher/rancher"
    )]
    pub image: String,
    #[clap(
        long = "tag",
        short = 't',
        about = "Specify image tag for Rancher (e.g. latest, stable)",
        default_value = "latest"
    )]
    pub image_tag: String,
    #[clap(flatten)]
    pub common: LaunchFlags,
}

#[derive(Clap, Debug)]
pub struct Stop {
    #[clap(
        long,
        short,
        about = "Select all Rancher containers (can be combined with the \"--delete\" flag)"
    )]
    pub all: bool,
    #[clap(about = "ID of the Rancher container running")]
    pub container_id: Option<String>,
    #[clap(
        long,
        about = "Delete container after stopping it (warning: this flag will permanently delete selected container(s))"
    )]
    pub delete: bool,
    #[clap(long, about = "Display containers to be acted on")]
    pub dry_run: bool,
}

#[derive(Clap, Debug)]
pub struct Upgrade {
    #[clap(about = "ID of the Rancher container running")]
    pub container_id: String,
    #[clap(about = "New tag for the upgrade (e.g. latest, stable)")]
    pub tag: String,
    #[clap(
        long,
        short,
        about = "Specify Rancher image for upgrade (defaults to the image in use)"
    )]
    pub image: Option<String>,
    #[clap(flatten)]
    pub common: LaunchFlags,
}

#[derive(Clap, Debug)]
pub struct Version {
    #[clap(long, short, about = "Display only the version tag")]
    pub short: bool,
}

// Subcommands using these flags should likely be "boxed": https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
#[derive(Clap, Debug)]
pub struct LaunchFlags {
    #[clap(
        long,
        about = "Set audit log level (for use with the \"--audit-log\" flag)",
        default_value = "1"
    )]
    pub audit_level: usize,
    #[clap(long, about = "Host path to audit log destination")]
    pub audit_log: Option<String>,
    #[clap(long, short, about = "Set bootstrap password (>=v2.6)")]
    pub bootstrap_password: Option<String>,
    #[clap(long, about = "Path to the certificate authority’s certificate")]
    pub ca_certs: Option<String>,
    #[clap(long, about = "Domain address (used for Let’s Encrypt)")]
    pub domain_address: Option<String>,
    #[clap(long, about = "Enable restricted default admin")]
    pub enable_restricted_default_admin: bool,
    #[clap(
        long,
        visible_alias = "fp",
        about = "Always pull the Rancher image, even if it exists locally"
    )]
    pub force_pull: bool,
    #[clap(
        long,
        about = "Path to your full certificate chain",
        requires = "private-key"
    )]
    pub full_chain: Option<String>,
    #[clap(
        long,
        about = "Host port mapped to container port 80",
        default_value = "80"
    )]
    pub host80: String,
    #[clap(
        long,
        about = "Host port mapped to container port 443",
        default_value = "443"
    )]
    pub host443: String,
    #[clap(long, about = "Path to CA root certificate for validating services")]
    pub host_certs: Option<String>,
    #[clap(
        long,
        short,
        about = "Disable CA certs (useful when using localhost tunneling services, like ngrok)"
    )]
    pub no_cacerts: bool,
    #[clap(
        long,
        about = "Host path for saving Rancher's persistent data from the embedded etcd database"
    )]
    pub persistent_data: Option<String>,
    #[clap(
        long,
        about = "Path to the private key for your certificate",
        requires = "full-chain"
    )]
    pub private_key: Option<String>,
    #[clap(long, about = "Proxy URL with port (e.g. http://127.0.0.1:1234)")]
    pub proxy: Option<String>,
    #[clap(long, about = "Set TLS ciphers")]
    pub tls_ciphers: Option<String>,
    #[clap(long, about = "Set TLS minimum version")]
    pub tls_min_version: Option<String>,
}
