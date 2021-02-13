use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::SystemTime;

use crate::jobs::Job;
use crate::resources::{Resources,NetworkResource};
use crate::tasks::TaskState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Allocation {
  pub id: String,
  pub namespace: String,
  pub eval_id: String,
  pub name: String,
  pub node_id: String,
  pub job: Job,
  pub task_group: String,
  pub resources: Resources,
  pub task_resources: HashMap<String,Resources>,
  pub allocated_resources: AllocatedResources,
  pub services: HashMap<String,String>,
  pub metrics: AllocationMetric,
  pub desired_state: String,
  pub desired_description: String,
  pub desired_transition: DesiredTransition,
  pub client_status: String,
  pub client_description: String,
  pub task_states: HashMap<String,TaskState>,
  pub deployment_id: String,
  pub deployment_status: AllocDeploymentStatus,
  pub followup_eval_id: String,
  pub previous_allocation: String,
  pub next_allocation: String,
  pub reschedule_tracker: RescheduleTracker,
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
  pub timestamp: SystemTime,
  pub canary: bool,
  pub modify_index: u64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocationMetric {
  pub nodes_evaluated: i64,
  pub nodes_filtered: i64,
  pub nodes_available: HashMap<String,i64>,
  pub class_filtered: HashMap<String,i64>,
  pub constraint_filtered: HashMap<String,i64>,
  pub nodes_exhausted: i64,
  pub class_exhausted: HashMap<String,i64>,
  pub dimensions_exhausted: HashMap<String,i64>,
  pub quota_exhausted: Vec<String>,
  // NOTE: Deprecated fields omitted
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeScoreMeta {
  pub node_id: String,
  pub scores: HashMap<String, f64>,
  pub norm_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocatedResources {
  pub tasks: HashMap<String, AllocatedTaskResources>,
  pub shared: AllocatedSharedResources,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocatedTaskResources {
  pub cpu: AllocatedCpuResources,
  pub memory: AllocatedMemoryResources,
  pub networks: Vec<NetworkResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocatedSharedResources {
  pub disk_mb: i64,
  pub networks: Vec<NetworkResource>,
  pub ports: Vec<PortMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PortMapping {
  pub label: String,
  pub value: i32,
  pub to: i32,
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
  pub memory_mb: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RescheduleTracker {
  pub events: Vec<RescheduleEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RescheduleEvent {
  pub reschedule_time: i64,
  pub prev_alloc_id: String,
  pub prev_node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DesiredTransition {
  pub migrate: bool,
  pub reschedule: bool,
}







