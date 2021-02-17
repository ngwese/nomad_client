use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::rust::default_on_null;
use serde_with::{serde_as, TimestampSeconds};

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use crate::allocations::AllocationMetric;
use crate::serde_helpers::hashi_duration;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Evaluation {
    #[serde(rename = "ID")]
    pub id: String,
    pub priority: i64,
    #[serde(rename = "Type")]
    pub evaluation_type: String,
    pub triggered_by: String,
    pub namespace: String,
    #[serde(rename = "JobID")]
    pub job_id: String,
    pub job_modify_index: u64,
    #[serde(rename = "NodeID")]
    pub node_id: String,
    pub node_modify_index: u64,
    #[serde(rename = "DeploymentID")]
    pub deployment_id: String,
    pub status: String,
    pub status_description: String,
    #[serde(deserialize_with = "hashi_duration::deserialize")]
    pub wait: Option<Duration>, // FIXME: deserializer expects Option type but this field might not be optional
    pub wait_until: DateTime<Utc>,
    pub next_eval: String,
    pub previous_eval: String,
    pub blocked_eval: String,
    #[serde(
        rename = "FailedTGAllocs",
        deserialize_with = "default_on_null::deserialize"
    )]
    pub failed_task_group_allocs: HashMap<String, AllocationMetric>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub class_eligibility: HashMap<String, bool>,
    pub escaped_computed_class: bool,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub queued_allocations: HashMap<String, i64>,
    pub snapshot_index: u64,
    pub create_index: u64,
    pub modify_index: u64,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub create_time: SystemTime,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub modify_time: SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_evaluation() {
        let js = r#"
        {
            "ID": "f623897e-17d1-828c-c2b5-10a892a97ca0",
            "Namespace": "default",
            "Priority": 50,
            "Type": "service",
            "TriggeredBy": "deployment-watcher",
            "JobID": "example",
            "JobModifyIndex": 0,
            "NodeID": "",
            "NodeModifyIndex": 0,
            "DeploymentID": "98605b0e-87da-e425-14e2-31d0c38cf06a",
            "Status": "complete",
            "StatusDescription": "",
            "Wait": 0,
            "WaitUntil": "0001-01-01T00:00:00Z",
            "NextEval": "",
            "PreviousEval": "",
            "BlockedEval": "",
            "FailedTGAllocs": null,
            "ClassEligibility": null,
            "QuotaLimitReached": "",
            "EscapedComputedClass": false,
            "AnnotatePlan": false,
            "QueuedAllocations": {
            "cache": 0
            },
            "LeaderACL": "",
            "SnapshotIndex": 33,
            "CreateIndex": 33,
            "ModifyIndex": 35,
            "CreateTime": 1613538639038711000,
            "ModifyTime": 1613538639295373000
        }
        "#;

        let evaluation: Evaluation = serde_json::from_str(&js).expect("deserialize failed");
        assert_eq!(
            evaluation.deployment_id,
            "98605b0e-87da-e425-14e2-31d0c38cf06a"
        );
        println!("evaluation = {:?}", evaluation);
    }
}
/*
*/
