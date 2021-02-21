use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::rust::default_on_null;

use std::collections::HashMap;
use std::time::Duration;

use super::constraint::Constraint;
use super::csi::CSIMountOptions;
use super::jobs::UpdateStrategy;
use super::resources::{NetworkResource, Resources};
use super::scaling::ScalingPolicy;
use super::serde_helpers::hashi_duration;
use super::services::Service;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Affinity {
    pub l_target: Option<String>,
    pub r_target: Option<String>,
    pub operand: Option<String>, // FIXME: Use Operand enum
    pub weight: Option<i8>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct RestartPolicy {
    #[serde(with = "hashi_duration")]
    pub interval: Option<Duration>,
    pub attempts: Option<i64>,
    #[serde(with = "hashi_duration")]
    pub delay: Option<Duration>,
    pub mode: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct VolumeRequest {
    pub name: String,
    #[serde(rename = "Type")]
    pub volume_type: Option<String>,
    pub source: Option<String>,
    pub read_only: Option<bool>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub mount_options: Vec<CSIMountOptions>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub extra_key_hcl: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct TaskState {
    pub state: String,
    pub failed: bool,
    pub restarts: u64,
    pub last_restart: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub events: Vec<TaskEvent>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct TaskEvent {
    #[serde(rename = "Type")]
    pub event_type: String,
    pub time: i64,
    pub display_message: String,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub details: HashMap<String, String>,
    pub message: String,
    // NOTE: Deprecated fields omitted
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct DispatchPayloadConfig {
    pub file: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct TaskLifecycle {
    pub hook: Option<String>,
    pub sidecar: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Task {
    pub name: String,
    pub driver: Option<String>,
    pub user: Option<String>,
    pub lifecycle: Option<TaskLifecycle>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub config: HashMap<String, serde_json::Value>, // FIXME: Figure out value type
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub constraints: Vec<Constraint>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub affinities: Vec<Affinity>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub env: HashMap<String, String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub services: Vec<Service>,
    pub resources: Option<Resources>,
    pub restart_policy: Option<RestartPolicy>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub meta: HashMap<String, String>,
    #[serde(with = "hashi_duration")]
    pub kill_timeout: Option<Duration>,
    pub log_config: Option<LogConfig>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub artifacts: Vec<TaskArtifact>,
    pub vault: Option<Vault>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub templates: Vec<Template>,
    pub dispatch_payload_config: Option<DispatchPayloadConfig>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub volume_mounts: Vec<VolumeMount>,
    pub csi_plugin_config: Option<TaskCSIPluginConfig>,
    pub leader: bool,
    #[serde(with = "hashi_duration")]
    pub shutdown_delay: Option<Duration>,
    pub kill_signal: Option<String>,
    pub kind: Option<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub scaling_policy: Vec<ScalingPolicy>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct TaskArtifact {
    pub getter_source: Option<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub getter_options: HashMap<String, String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub getter_headers: HashMap<String, String>,
    pub getter_mode: Option<String>,
    pub relative_dest: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct VolumeMount {
    pub volume: Option<String>,
    pub destination: Option<String>,
    pub read_only: bool,
    pub propagation_mode: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct TaskGroup {
    pub name: String,
    pub count: Option<i64>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub constraints: Vec<Constraint>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub affinities: Vec<Affinity>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub tasks: Vec<Task>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub spreads: Vec<Spread>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub volumes: HashMap<String, VolumeRequest>,
    pub restart_policy: Option<RestartPolicy>,
    pub reschedule_policy: Option<ReschedulePolicy>,
    pub ephemeral_disk: Option<EphemeralDisk>,
    pub update: Option<UpdateStrategy>,
    pub migrate: Option<MigrateStrategy>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub networks: Vec<NetworkResource>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub meta: HashMap<String, String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub services: Vec<Service>,
    #[serde(with = "hashi_duration")]
    pub shutdown_delay: Option<Duration>,
    #[serde(with = "hashi_duration")]
    pub stop_after_client_disconnect: Option<Duration>,
    pub scaling: Option<ScalingPolicy>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct MigrateStrategy {
    pub max_parallel: Option<i64>,
    pub health_check: Option<String>,
    #[serde(with = "hashi_duration")]
    pub min_healthy_time: Option<Duration>,
    #[serde(with = "hashi_duration")]
    pub healthy_deadline: Option<Duration>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ReschedulePolicy {
    pub attempts: Option<i64>,
    #[serde(with = "hashi_duration")]
    pub interval: Option<Duration>,
    #[serde(with = "hashi_duration")]
    pub delay: Option<Duration>,
    pub delay_function: Option<String>,
    #[serde(with = "hashi_duration")]
    pub max_delay: Option<Duration>,
    pub unlimited: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Spread {
    pub attribute: Option<String>,
    pub weight: Option<i8>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub spread_target: Vec<SpreadTarget>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SpreadTarget {
    #[serde(rename = "Label")]
    pub value: String,
    pub percent: Option<u8>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct EphemeralDisk {
    pub sticky: Option<bool>,
    pub migrate: Option<bool>,
    #[serde(rename = "SizeMB")]
    pub size_mb: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct LogConfig {
    pub max_files: Option<i64>,
    #[serde(rename = "MaxFileSizeMB")]
    pub max_file_size_mb: Option<i64>,
}

impl Default for LogConfig {
    fn default() -> LogConfig {
        LogConfig {
            max_files: Some(10),
            max_file_size_mb: Some(10),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Template {
    pub source_path: Option<String>,
    pub dest_path: Option<String>,
    pub embedded_tmpl: Option<String>,
    pub change_mode: Option<String>,
    pub change_signal: Option<String>,
    #[serde(with = "hashi_duration")]
    pub splay: Option<Duration>,
    pub perms: Option<String>,
    pub left_delim: Option<String>,
    pub right_delim: Option<String>,
    pub envvars: bool,
    #[serde(with = "hashi_duration")]
    pub vault_grace: Option<Duration>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Vault {
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub policies: Vec<String>,
    pub namespace: Option<String>,
    pub env: bool,
    pub change_mode: Option<String>,
    pub change_signal: Option<String>,
}

pub type CSIPluginType = String;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskCSIPluginConfig {
    pub id: Option<String>,
    #[serde(rename = "Type")]
    pub plugin_type: Option<CSIPluginType>,
    pub mount_dir: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_task_group() {
        let js = r#"{
            "Name": "cache",
            "Count": 3,
            "Constraints": null,
            "Affinities": null,
            "Tasks": [
                {
                    "Name": "redis",
                    "Driver": "docker",
                    "User": "",
                    "Lifecycle": null,
                    "Config": {
                        "image": "redis:4.0",
                        "ports": [
                            "db"
                        ]
                    },
                    "Constraints": null,
                    "Affinities": null,
                    "Env": null,
                    "Services": null,
                    "Resources": {
                        "CPU": 500,
                        "MemoryMB": 256,
                        "DiskMB": null,
                        "Networks": null,
                        "Devices": null,
                        "IOPS": null
                    },
                    "RestartPolicy": null,
                    "Meta": null,
                    "KillTimeout": null,
                    "LogConfig": null,
                    "Artifacts": null,
                    "Vault": null,
                    "Templates": null,
                    "DispatchPayload": null,
                    "VolumeMounts": null,
                    "Leader": false,
                    "ShutdownDelay": 0,
                    "KillSignal": "",
                    "Kind": "",
                    "ScalingPolicies": null
                }
            ],
            "Spreads": null,
            "Volumes": null,
            "RestartPolicy": {
                "Interval": 1800000000000,
                "Attempts": 2,
                "Delay": 15000000000,
                "Mode": "fail"
            },
            "ReschedulePolicy": null,
            "EphemeralDisk": {
                "Sticky": null,
                "Migrate": null,
                "SizeMB": 300
            },
            "Update": null,
            "Migrate": null,
            "Networks": [
                {
                    "Mode": "",
                    "Device": "",
                    "CIDR": "",
                    "IP": "",
                    "DNS": null,
                    "ReservedPorts": null,
                    "DynamicPorts": [
                        {
                            "Label": "db",
                            "Value": 0,
                            "To": 6379,
                            "HostNetwork": ""
                        }
                    ],
                    "MBits": null
                }
            ],
            "Meta": null,
            "Services": [
                {
                    "Id": "",
                    "Name": "redis-cache",
                    "Tags": [
                        "global",
                        "cache"
                    ],
                    "CanaryTags": null,
                    "EnableTagOverride": false,
                    "PortLabel": "db",
                    "AddressMode": "",
                    "Checks": null,
                    "CheckRestart": null,
                    "Connect": null,
                    "Meta": null,
                    "CanaryMeta": null,
                    "TaskName": ""
                }
            ],
            "ShutdownDelay": null,
            "StopAfterClientDisconnect": null,
            "Scaling": null
        }"#;

        let _tg: TaskGroup = serde_json::from_str(&js).expect("deserialize failed");
    }
}
