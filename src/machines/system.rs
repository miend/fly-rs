use crate::machines::TimeoutConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InitConfig {
    pub cmd: Option<Vec<String>>,
    pub entrypoint: Option<Vec<String>>,
    pub exec: Option<Vec<String>>,
    pub kernel_args: Option<Vec<String>>,
    pub swap_size_mb: Option<u64>,
    pub tty: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MountConfig {
    pub add_size_gb: Option<u64>,
    pub encrypted: Option<bool>,
    pub extend_threshold_percent: Option<u64>,
    pub name: Option<String>,
    pub path: String,
    pub size_gb: Option<u64>,
    pub size_gb_limit: Option<u64>,
    pub volume: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileConfig {
    pub guest_path: String,
    pub mode: Option<u32>,
    pub raw_value: Option<String>,
    pub secret_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StaticConfig {
    pub guest_path: String,
    pub url_prefix: String,
    pub index_document: Option<String>,
    pub tigris_bucket: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MetricsConfig {
    pub port: u16,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StopConfig {
    pub signal: Option<String>,
    pub timeout: Option<TimeoutConfig>,
}
