use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use crate::constraint::Constraint;
use crate::csi::CSIMountOptions;
use crate::jobs::UpdateStrategy;
use crate::resources::NetworkResource;
use crate::scaling::ScalingPolicy;
use crate::services::Service;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Affinity {
    pub l_target: Option<String>,
    pub r_target: Option<String>,
    pub operand: Option<String>, // FIXME: Use Operand enum
    pub weight: Option<i8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RestartPolicy {
    pub interval: Option<Duration>,
    pub attempts: Option<i64>,
    pub delay: Option<Duration>,
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeRequest {
    pub name: String,
    #[serde(rename = "Type")]
    pub volume_type: Option<String>,
    pub source: Option<String>,
    pub read_only: Option<bool>,
    pub mount_options: Vec<CSIMountOptions>,
    pub extra_key_hcl: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskState {
    pub state: String,
    pub failed: bool,
    pub restarts: u64,
    pub last_restart: SystemTime,
    pub started_at: SystemTime,
    pub finished_at: SystemTime,
    pub events: Vec<TaskEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskEvent {
    #[serde(rename = "Type")]
    pub event_type: String,
    pub time: i64,
    pub display_message: String,
    pub details: HashMap<String, String>,
    pub message: String,
    // NOTE: Deprecated fields omitted
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Task {
    // FIXME:
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskGroup {
    pub name: String,
    pub count: Option<i64>,
    pub constraints: Vec<Constraint>,
    pub affinities: Vec<Affinity>,
    pub tasks: Vec<Task>,
    pub spreads: Vec<Spread>,
    pub volumes: HashMap<String, VolumeRequest>,
    pub restart_policy: Option<RestartPolicy>,
    pub reschedule_policy: Option<ReschedulePolicy>,
    pub ephemeral_disk: Option<EphemeralDisk>,
    pub update: Option<UpdateStrategy>,
    pub migrate: Option<MigrateStrategy>,
    pub networks: Vec<NetworkResource>,
    pub meta: HashMap<String, String>,
    pub services: Vec<Service>,
    pub shutdown_delay: Option<Duration>,
    pub stop_after_client_disconnect: Option<Duration>,
    pub scaling: Option<ScalingPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MigrateStrategy {
    pub max_parallel: Option<i64>,
    pub health_check: Option<String>,
    pub min_healthy_time: Option<Duration>,
    pub healthy_deadline: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReschedulePolicy {
    pub attempts: Option<i64>,
    pub interval: Option<Duration>,
    pub delay: Option<Duration>,
    pub delay_function: Option<String>,
    pub max_delay: Option<Duration>,
    pub unlimited: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Spread {
    pub attribute: Option<String>,
    pub weight: Option<i8>,
    pub spread_target: Vec<SpreadTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpreadTarget {
    #[serde(rename = "Label")]
    pub value: String,
    pub percent: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EphemeralDisk {
    pub sticky: Option<bool>,
    pub migrate: Option<bool>,
    #[serde(rename = "Size")]
    pub size_mb: Option<i64>,
}
