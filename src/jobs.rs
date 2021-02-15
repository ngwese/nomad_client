use serde::{Deserialize, Serialize};
use serde_with::rust::default_on_null;

use std::collections::HashMap;
use std::time::Duration;

use crate::constraint::Constraint;
use crate::serde_helpers::hashi_duration;
use crate::tasks::{Affinity, MigrateStrategy, ReschedulePolicy, Spread, TaskGroup};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobType {
    Service,
    Batch,
    System,
}

impl Default for JobType {
    fn default() -> JobType {
        JobType::Service
    }
}

#[allow(dead_code)]
pub const DEFAULT_NAMESPACE: &str = "default";
#[allow(dead_code)]
pub const GLOBAL_REGION: &str = "global";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct MultiregionStrategy {
    pub max_parallel: Option<i64>,
    pub on_failure: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct MultiregionRegion {
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub name: String,
    pub count: Option<i64>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub datacenters: Vec<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub meta: HashMap<String, String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Multiregion {
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub strategy: Vec<MultiregionStrategy>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub regions: Vec<MultiregionRegion>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PeriodicConfig {
    pub enabled: Option<bool>,
    #[serde(rename = "Cron")]
    pub spec: Option<String>,
    pub spec_type: Option<String>,
    pub prohibit_overlap: Option<bool>,
    pub time_zone: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct UpdateStrategy {
    #[serde(with = "hashi_duration")]
    pub stagger: Option<Duration>,
    pub max_parallel: Option<i64>,
    pub health_check: Option<String>,
    #[serde(with = "hashi_duration")]
    pub min_healthy_time: Option<Duration>,
    #[serde(with = "hashi_duration")]
    pub healthy_deadline: Option<Duration>,
    #[serde(with = "hashi_duration")]
    pub progress_deadline: Option<Duration>,
    pub canary: Option<i64>,
    pub auto_revert: Option<bool>,
    pub auto_promote: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ParameterizedJobConfig {
    pub payload: Option<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub meta_required: Vec<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub meta_optional: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Job {
    pub region: Option<String>,
    pub namespace: Option<String>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "Type")]
    pub job_type: Option<JobType>,
    pub priority: Option<i64>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub all_at_once: bool,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub datacenters: Vec<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub constriants: Vec<Constraint>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub affinities: Vec<Affinity>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub task_group: Vec<TaskGroup>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub update: Option<UpdateStrategy>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub multiregion: Option<Multiregion>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub spreads: Vec<Spread>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub periodic: Option<PeriodicConfig>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub parameterized_job: Option<ParameterizedJobConfig>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub reschedule: Option<ReschedulePolicy>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub migrate: Option<MigrateStrategy>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub meta: HashMap<String, String>,
    pub consul_token: Option<String>,
    pub vault_token: Option<String>,

    // Server managed fields
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub stop: bool,
    pub parent_id: Option<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub dispatched: bool,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub payload: Vec<u8>,
    pub vault_namespace: Option<String>,
    pub nomad_token_id: Option<String>,
    pub status: Option<String>,
    pub status_description: Option<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub stable: bool,
    pub version: Option<u64>,
    pub submit_time: Option<i64>,
    pub create_index: Option<u64>,
    pub modify_index: Option<u64>,
    pub job_modify_index: Option<u64>,
}

// JobSummary summarizes the state of the allocations of a job
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct JobSummary {
    #[serde(rename = "JobID")]
    pub job_id: String,
    pub namespace: String,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub summary: HashMap<String, TaskGroupSummary>,
    pub children: Option<JobChildrenSummary>,
    pub create_index: u64,
    pub modify_index: u64,
}

// JobChildrenSummary contains the summary of children job status
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct JobChildrenSummary {
    pub pending: i64,
    pub running: i64,
    pub dead: i64,
}

// TaskGroup summarizes the state of all the allocations of a particular
// TaskGroup
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct TaskGroupSummary {
    pub queued: i64,
    pub complete: i64,
    pub failed: i64,
    pub running: i64,
    pub starting: i64,
    pub lost: i64,
}

// JobListStub is used to return a subset of information about
// jobs during list operations
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct JobListStub {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "ParentID")]
    pub parent_id: String,
    pub name: String,
    pub namespace: Option<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub datacenters: Vec<String>,
    #[serde(rename = "Type")]
    pub job_type: JobType,
    pub priority: i64,
    pub periodic: bool,
    pub parameterized_job: bool,
    pub stop: bool,
    pub status: String,
    pub status_description: String,
    pub job_summary: Option<JobSummary>,
    pub create_index: u64,
    pub modify_index: u64,
    pub job_modify_index: u64,
    pub submit_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobSpec {
    pub job: Job,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_job_spec() {
        let js = r#"
        {
            "Job": {
                "Region": null,
                "Namespace": null,
                "ID": "example",
                "Name": "example",
                "Type": "service",
                "Priority": null,
                "AllAtOnce": null,
                "Datacenters": [
                    "dc1"
                ],
                "Constraints": null,
                "Affinities": null,
                "TaskGroups": [
                    {
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
                    }
                ],
                "Update": {
                    "Stagger": null,
                    "MaxParallel": 1,
                    "HealthCheck": "task_states",
                    "MinHealthyTime": 10000000000,
                    "HealthyDeadline": 180000000000,
                    "ProgressDeadline": 600000000000,
                    "Canary": 0,
                    "AutoRevert": false,
                    "AutoPromote": null
                },
                "Multiregion": null,
                "Spreads": null,
                "Periodic": null,
                "ParameterizedJob": null,
                "Reschedule": null,
                "Migrate": {
                    "MaxParallel": 1,
                    "HealthCheck": "checks",
                    "MinHealthyTime": 10000000000,
                    "HealthyDeadline": 300000000000
                },
                "Meta": null,
                "ConsulToken": null,
                "VaultToken": null,
                "Stop": null,
                "ParentID": null,
                "Dispatched": false,
                "Payload": null,
                "VaultNamespace": null,
                "NomadTokenID": null,
                "Status": null,
                "StatusDescription": null,
                "Stable": null,
                "Version": null,
                "SubmitTime": null,
                "CreateIndex": null,
                "ModifyIndex": null,
                "JobModifyIndex": null
            }
        }
        "#;

        let _job_spec: JobSpec = serde_json::from_str(&js).expect("deserialize failed");
    }
}
