use serde::{Deserialize, Serialize};

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
  Allocation,
  Deployment,
  Evaluation,
  Job,
  Node
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
  pub topic: Topic,
  pub event_type: String,
  pub key: String,
  pub filter_keys: Vec<String>,
  pub index: u64,
  pub payload: EventPayload,
}

#[derive(Debug)]
pub struct Events {
  pub index: u64,
  pub events: Vec<Event>,
  pub error: Box<dyn std::error::Error>,
}

#[cfg(test)]
mod tests {
}
