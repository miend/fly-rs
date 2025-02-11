use crate::machines::{
    MachineConfig,
    MachineRegion,
    // Checks, DnsConfig, FileConfig, GuestConfig, Header, InitConfig,
    // MetricsConfig, MountConfig, ProcessConfig, RestartPolicy, ServiceConfig, StaticConfig,
    // StopConfig, TimeoutConfig,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MachineListRequest {
    pub region: Option<MachineRegion>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MachineRequest {
    pub name: Option<String>,
    pub config: MachineConfig,
    pub region: Option<MachineRegion>,
    pub lease_ttl: Option<u64>,
    pub lsvd: Option<bool>,
    pub skip_launch: Option<bool>,
    pub skip_service_registration: Option<bool>,
}

impl MachineRequest {
    pub fn new(config: MachineConfig, name: Option<String>, region: Option<MachineRegion>) -> Self {
        Self {
            name: name,
            config,
            region: region,
            lease_ttl: None,
            lsvd: None,
            skip_launch: None,
            skip_service_registration: None,
        }
    }

    pub fn with_lease_ttl(mut self, lease_ttl: u64) -> Self {
        self.lease_ttl = Some(lease_ttl);
        self
    }

    pub fn with_lsvd(mut self, lsvd: bool) -> Self {
        self.lsvd = Some(lsvd);
        self
    }

    pub fn with_skip_launch(mut self, skip_launch: bool) -> Self {
        self.skip_launch = Some(skip_launch);
        self
    }

    pub fn with_skip_service_registration(mut self, skip_service_registration: bool) -> Self {
        self.skip_service_registration = Some(skip_service_registration);
        self
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MachineResponse {
    pub id: Option<String>,
    pub checks: Option<Vec<CheckResponse>>,
    pub config: Option<MachineConfig>,
    pub created_at: Option<String>,
    pub events: Option<Vec<EventResponse>>,
    pub host_status: Option<HostStatusEnum>,
    pub image_ref: Option<ImageRef>,
    pub incomplete_config: Option<Value>,
    pub instance_id: Option<String>,
    pub name: Option<String>,
    pub nonce: Option<String>,
    pub private_ip: Option<String>,
    pub region: Option<MachineRegion>,
    pub state: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventResponse {
    pub id: Option<String>,
    pub request: Option<Value>,
    pub source: Option<String>,
    pub status: Option<String>,
    pub timestamp: Option<u64>,
    pub event_type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HostStatusEnum {
    Ok,
    Unknown,
    Unreachable,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageRef {
    pub digest: Option<String>,
    pub labels: Option<Value>,
    pub registry: Option<String>,
    pub repository: Option<String>,
    pub tag: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckResponse {
    pub name: Option<String>,
    pub output: Option<String>,
    pub status: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CreateMachinesResponse {
    machines: Vec<MachineResponse>,
}
