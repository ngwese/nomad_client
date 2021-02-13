use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operand {
    DistinctProperty,  // "distinct_property"
    DistinctHosts,     // "distinct_hosts"
    Regex,             // "regexp"
    Version,           // "version"
    Semver,            // "semver"
    SetContains,       // "set_contains"
    SetContainsAll,    // "set_contains_all"
    SetContainsAny,    // "set_contains_any"
    AttributeIsSet,    // "is_set"
    AttributeIsNotSet, // "is_not_set"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Constraint {
    pub l_target: Option<String>,
    pub r_target: Option<String>,
    pub operand: Option<String>, // FIXME: Use Operand enum
}
