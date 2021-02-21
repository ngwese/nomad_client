use serde::{Deserialize, Serialize};
use serde_with::rust::default_on_null;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct CSIMountOptions {
    pub fs_type: Option<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub mount_flags: Vec<String>,
    #[serde(
        rename = "ExtraKeysHCL",
        deserialize_with = "default_on_null::deserialize"
    )]
    pub extra_keys_hcl: Vec<String>,
}
