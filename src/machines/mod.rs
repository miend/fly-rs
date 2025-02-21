pub mod api_manager;
pub mod checks;
pub mod endpoints;
pub mod machine;
pub mod networking;
pub mod process;
pub mod regions;
pub mod resources;
pub mod services;
pub mod system;

pub use api_manager::MachineManager;
pub use checks::{CheckKind, CheckType, Checks, Header, Protocol};
pub use endpoints::{EventResponse, MachineRequest, MachineResponse};
pub use machine::{MachineConfig, MachineState};
pub use networking::{DnsConfig, DnsForwardRule};
pub use process::{
    CommandResponse, EnvVarConfig, FieldRef, ProcessConfig, ProcessResponse, SecretConfig,
};
pub use regions::MachineRegion;
pub use resources::{CpuKind, GpuKind, GuestConfig, MachineRestart, RestartPolicy};
pub use services::ServiceConfig;
pub use system::{FileConfig, InitConfig, MetricsConfig, MountConfig, StaticConfig, StopConfig};

use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct TimeoutConfig {
    pub duration: u64,
}

impl TimeoutConfig {
    pub fn new(duration: u64) -> Self {
        Self { duration }
    }
}

impl<'de> Deserialize<'de> for TimeoutConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// Enum used to deserialize fly machine timeout durations, which could be defined as either
        /// seconds (integer), or Golang duration string.
        #[derive(Deserialize, Debug)]
        #[serde(untagged)]
        enum TimeoutDurationType {
            GoDuration(String),
            Seconds(u64),
        }

        let intermediate = TimeoutDurationType::deserialize(deserializer)?;

        match intermediate {
            TimeoutDurationType::GoDuration(s) => {
                let (num_str, unit) = s.split_at(s.len() - 1);
                let num: u64 = num_str.parse().map_err(de::Error::custom)?;

                let duration = match unit {
                    "s" => num,
                    "m" => num * 60,
                    "h" => num * 3600,
                    "d" => num * 86400,
                    _ => return Err(de::Error::custom("Invalid time unit")),
                };

                Ok(TimeoutConfig::new(duration))
            }
            TimeoutDurationType::Seconds(duration) => Ok(TimeoutConfig::new(duration)),
        }
    }
}
