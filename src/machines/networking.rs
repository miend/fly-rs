use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DnsConfig {
    pub dns_forward_rules: Option<Vec<DnsForwardRule>>,
    pub hostname: Option<String>,
    pub hostname_fqdn: Option<String>,
    pub nameservers: Option<Vec<String>>,
    pub options: Option<Vec<String>>,
    pub searches: Option<Vec<String>>,
    pub skip_registration: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DnsForwardRule {
    pub source: String,
    pub destination: String,
    pub protocol: Option<String>,
}
