use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::constraint::Constraint;
use crate::tasks::{Affinity, MigrateStrategy, ReschedulePolicy, Spread, TaskGroup};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobType {
    Service,
    Batch,
    System,
}

#[allow(dead_code)]
pub const DEFAULT_NAMESPACE: &str = "default";
#[allow(dead_code)]
pub const GLOBAL_REGION: &str = "global";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MultiregionStrategy {
    pub max_parallel: Option<i64>,
    pub on_failure: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MultiregionRegion {
    pub name: String,
    pub count: Option<i64>,
    pub datacenters: Vec<String>,
    pub meta: HashMap<String, String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Multiregion {
    pub strategy: Vec<MultiregionStrategy>,
    pub regions: Vec<MultiregionRegion>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PeriodicConfig {
    pub enabled: Option<bool>,
    #[serde(rename = "Cron")]
    pub spec: Option<String>,
    pub spec_type: Option<String>,
    pub prohibit_overlap: Option<bool>,
    pub time_zone: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateStrategy {
    pub stagger: Option<Duration>,
    pub max_parallel: Option<i64>,
    pub health_check: Option<String>,
    pub min_healthy_time: Option<Duration>,
    pub healthy_deadline: Option<Duration>,
    pub progress_deadline: Option<Duration>,
    pub canary: Option<i64>,
    pub auto_revert: Option<bool>,
    pub auto_promote: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParameterizedJobConfig {
    pub payload: Option<String>,
    pub meta_required: Vec<String>,
    pub meta_optional: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Job {
    pub region: Option<String>,
    pub namespace: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "Type")]
    pub job_type: Option<JobType>,
    pub priority: Option<i64>,
    pub all_at_once: bool,
    pub datacenters: Vec<String>,
    pub constriants: Vec<Constraint>,
    pub affinities: Vec<Affinity>,
    pub task_group: Vec<TaskGroup>,
    pub update: UpdateStrategy,
    pub multiregion: Multiregion,
    pub spreads: Vec<Spread>,
    pub periodic: PeriodicConfig,
    pub parameterized_job: ParameterizedJobConfig,
    pub reschedule: ReschedulePolicy,
    pub migrate: MigrateStrategy,
    pub meta: HashMap<String, String>,
    pub consul_token: Option<String>,
    pub vault_token: Option<String>,

    // Server managed fields
    pub stop: bool,
    pub parent_id: Option<String>,
    pub dispatched: bool,
    pub payload: Vec<u8>,
    pub vault_namespace: Option<String>,
    pub nomad_token_id: Option<String>,
    pub status: Option<String>,
    pub status_description: Option<String>,
    pub stable: bool,
    pub version: Option<u64>,
    pub submit_time: Option<i64>,
    pub create_index: Option<u64>,
    pub modify_index: Option<u64>,
    pub job_modify_index: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobSpec {
    pub job: Job,
}

#[cfg(test)]
mod tests {}
