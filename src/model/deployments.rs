use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::rust::default_on_null;
use serde_with::serde_as;

use std::collections::HashMap;
use std::time::Duration;

use super::serde_helpers::hashi_duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Deployment {
    #[serde(rename = "ID")]
    pub id: String,
    pub namespace: String,
    #[serde(rename = "JobID")]
    pub job_id: String,
    pub job_version: u64,
    pub job_modify_index: u64,
    pub job_spec_modify_index: u64,
    pub job_create_index: u64,
    pub is_multiregion: bool,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub task_groups: HashMap<String, DeploymentState>,
    pub status: String,
    pub status_description: String,
    pub create_index: u64,
    pub modify_index: u64,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeploymentState {
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub placed_canaries: Vec<String>,
    pub auto_revert: bool,
    #[serde(deserialize_with = "hashi_duration::deserialize")]
    pub progress_deadline: Option<Duration>, // FIXME: this is not optional in the original API but the deserializer expects it
    pub require_progress_by: DateTime<Utc>,
    pub promoted: bool,
    pub desired_canaries: i64,
    pub desired_total: i64,
    pub placed_allocs: i64,
    pub healthy_allocs: i64,
    pub unhealthy_allocs: i64,
}
