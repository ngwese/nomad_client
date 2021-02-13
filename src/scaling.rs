use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScalingPolicy {
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub policy: HashMap<String, String>, // FIXME: what are these
    pub enabled: Option<bool>,
    #[serde(rename = "Type")]
    pub policy_type: Option<String>,

    // Server managed fields
    pub id: String,
    pub namespace: String,
    pub target: HashMap<String, String>,
    pub create_index: u64,
    pub modify_index: u64,
}
