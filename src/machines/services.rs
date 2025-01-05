use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServiceConfig {
    pub autostart: Option<bool>,
    pub autostop: Option<String>,
    pub concurrency: Option<ConcurrencyConfig>,
    pub ports: Option<Vec<MachinePort>>,
    pub internal_port: Option<u16>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AutostopEnum {
    Off,
    Stop,
    Suspend,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConcurrencyConfig {
    pub hard_limit: Option<u32>,
    pub soft_limit: Option<u32>,
    pub concurrency_type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ConcurrencyTypeEnum {
    Connections,
    Requests,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MachinePort {
    pub end_port: Option<u16>,
    pub force_https: Option<bool>,
    pub handlers: Option<Vec<String>>,
    pub http_options: Option<HttpOptions>,
    pub proxy_proto_options: Option<ProxyProtoOptions>,
    pub start_port: Option<u16>,
    pub tls_options: Option<TlsOptions>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpOptions {
    pub compress: Option<bool>,
    pub h2_backend: Option<bool>,
    pub headers_read_timeout: Option<u64>,
    pub idle_timeout: Option<u64>,
    pub response: Option<ResponseOptions>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResponseOptions {
    pub headers: Option<HashMap<String, String>>,
    pub pristine: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProxyProtoOptions {
    pub version: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TlsOptions {
    pub alpn: Option<Vec<String>>,
    pub default_self_signed: Option<bool>,
    pub versions: Option<Vec<String>>,
}
