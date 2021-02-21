use serde::{Deserialize, Serialize};
use serde_with::rust::default_on_null;

use super::allocations::Allocation;
use super::deployments::Deployment;
use super::evaluations::Evaluation;
use super::jobs::Job;
use super::nodes::Node;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Topic {
    Deployment,
    Evaluation,
    Allocation,
    Job,
    Node,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventPayload {
    Allocation(Allocation),
    Deployment(Deployment),
    Evaluation(Evaluation),
    Job(Job),
    Node(Node),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    pub topic: Topic,
    #[serde(rename = "Type")]
    pub event_type: String,
    pub key: String,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub filter_keys: Vec<String>,
    pub index: u64,
    pub payload: EventPayload,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Events {
    pub index: u64,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub events: Vec<Event>,
    #[serde(rename = "Err")]
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {}
