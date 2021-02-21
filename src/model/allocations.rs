use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::rust::default_on_null;

use std::collections::HashMap;
use std::net::IpAddr;

use super::jobs::Job;
use super::resources::{NetworkResource, Resources};
use super::tasks::TaskState;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Allocation {
    #[serde(rename = "ID")]
    pub id: String,
    pub namespace: String,
    #[serde(rename = "EvalID")]
    pub eval_id: String,
    pub name: String,
    #[serde(rename = "NodeID")]
    pub node_id: String,
    pub node_name: String,
    #[serde(rename = "JobID")]
    pub job_id: String,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub job: Option<Job>,
    pub task_group: String,
    pub resources: Resources,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub task_resources: HashMap<String, Resources>,
    pub allocated_resources: Option<AllocatedResources>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub services: Option<HashMap<String, String>>,
    pub metrics: AllocationMetric,
    pub desired_state: String,
    pub desired_description: String,
    pub desired_transition: DesiredTransition,
    pub client_status: String,
    pub client_description: String,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub task_states: HashMap<String, TaskState>,
    pub deployment_id: String,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub deployment_status: Option<AllocDeploymentStatus>,
    #[serde(rename = "FollowupEvalID")]
    pub followup_eval_id: String,
    pub previous_allocation: String,
    pub next_allocation: String,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub reschedule_tracker: Option<RescheduleTracker>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub preempted_allocations: Vec<String>,
    pub preempted_by_allocation: String,
    pub create_index: u64,
    pub modify_index: u64,
    pub alloc_modify_index: u64,
    pub create_time: i64,
    pub modify_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocDeploymentStatus {
    pub healthy: bool,
    pub timestamp: DateTime<Utc>,
    pub canary: bool,
    pub modify_index: u64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocationMetric {
    pub nodes_evaluated: i64,
    pub nodes_filtered: i64,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub nodes_available: HashMap<String, i64>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub class_filtered: HashMap<String, i64>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub constraint_filtered: HashMap<String, i64>,
    pub nodes_exhausted: i64,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub class_exhausted: HashMap<String, i64>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub dimension_exhausted: HashMap<String, i64>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub quota_exhausted: Vec<String>,
    // NOTE: Deprecated fields omitted
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeScoreMeta {
    pub node_id: String,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub scores: HashMap<String, f64>,
    pub norm_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocatedResources {
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub tasks: HashMap<String, AllocatedTaskResources>,
    pub shared: AllocatedSharedResources,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocatedTaskResources {
    pub cpu: AllocatedCpuResources,
    pub memory: AllocatedMemoryResources,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub networks: Vec<NetworkResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocatedSharedResources {
    #[serde(rename = "DiskMB")]
    pub disk_mb: i64,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub networks: Vec<NetworkResource>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub ports: Vec<PortMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PortMapping {
    pub label: String,
    pub value: i32,
    pub to: i32,
    #[serde(rename = "HostIP")]
    pub host_ip: IpAddr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocatedCpuResources {
    pub cpu_shares: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocatedMemoryResources {
    #[serde(rename = "MemoryMB")]
    pub memory_mb: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RescheduleTracker {
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub events: Vec<RescheduleEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RescheduleEvent {
    pub reschedule_time: i64,
    #[serde(rename = "PrevAllocID")]
    pub prev_alloc_id: String,
    #[serde(rename = "PrevNodeID")]
    pub prev_node_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DesiredTransition {
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub migrate: Option<bool>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub reschedule: Option<bool>,
}
