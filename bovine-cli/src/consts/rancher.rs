pub const AUDIT_LOG: &str = "/var/log/auditlog";
pub const CA_CERTS: &str = "/etc/rancher/ssl/cacerts.pem";
pub const CATTLE_RESTRICTED_DEFAULT_ADMIN: &str = "CATTLE_RESTRICTED_DEFAULT_ADMIN=\"true\"";
pub const CONTAINER_CERTS: &str = "/container/certs";
pub const CONTAINER_PORT_80: &str = "80/tcp";
pub const CONTAINER_PORT_443: &str = "443/tcp";
pub const FULL_CHAIN: &str = "/etc/rancher/ssl/cert.pem";
pub const NO_PROXY: &str = "NO_PROXY=\"localhost,127.0.0.1,0.0.0.0,10.0.0.0/8,cattle-system.svc,192.168.10.0/24,.svc,.cluster.local,example.com\"";
pub const PRIVATE_KEY: &str = "/etc/rancher/ssl/key.pem";
pub const SSL_CERT_DIR: &str = "SSL_CERT_DIR=\"/container/certs\"";
pub const VAR_LIB_RANCHER: &str = "/var/lib/rancher";

pub const BOOTSTRAP_PASSWORD_SEARCH_TERM: &str = "Bootstrap Password: ";
