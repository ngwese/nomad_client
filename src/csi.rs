use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CSIMountOptions {
    pub fs_type: Option<String>,
    pub mount_flags: Vec<String>,
    pub extra_keys_hcl: Vec<String>,
}
