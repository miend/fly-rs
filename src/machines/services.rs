use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServiceConfig {
    pub autostart: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_autostop")]
    pub autostop: Option<AutostopSetting>,
    pub concurrency: Option<ConcurrencyConfig>,
    pub ports: Option<Vec<MachinePort>>,
    pub protocol: Option<NetworkProtocol>,
    pub internal_port: Option<u16>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum AutostopSetting {
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ConcurrencyType {
    Connections,
    Requests,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NetworkProtocol {
    Tcp,
    Udp,
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

fn deserialize_autostop<'de, D>(deserializer: D) -> Result<Option<AutostopSetting>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match value {
        // Older fly apps will have boolean values in their config
        serde_json::Value::Bool(false) => Ok(Some(AutostopSetting::Off)),
        serde_json::Value::Bool(true) => Ok(Some(AutostopSetting::Stop)),
        // Newer fly apps use the string variant
        serde_json::Value::String(ref s) if s.eq_ignore_ascii_case("off") => {
            Ok(Some(AutostopSetting::Off))
        }
        serde_json::Value::String(ref s) if s.eq_ignore_ascii_case("stop") => {
            Ok(Some(AutostopSetting::Stop))
        }
        serde_json::Value::String(ref s) if s.eq_ignore_ascii_case("suspend") => {
            Ok(Some(AutostopSetting::Suspend))
        }
        _ => Err(serde::de::Error::custom(
            "Invalid value for AutostopSetting",
        )),
    }
}
