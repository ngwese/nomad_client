use serde::{Deserialize, Serialize};
use std::net::IpAddr;

use crate::constraint::Constraint;
use crate::tasks::Affinity;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Resources {
  pub cpu: Option<i64>,
  #[serde(rename = "Memory")]
  pub memory_mb: Option<i64>,
  #[serde(rename = "Disk")]
  pub disk_mb: Option<i64>,
  pub networks: Vec<NetworkResource>,
  pub devices: Vec<RequestedDevice>,
  // NOTE: Deprecated fields omitted
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Port {
  pub label: String,
  pub value: Option<i64>,
  pub to: Option<i64>,
  pub host_network: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DNSConfig {
  pub servers: Vec<String>,
  pub searches: Vec<String>,
  pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkResource {
  pub mode: Option<String>,
  pub device: Option<String>,
  pub cidr: Option<String>,
  pub ip: Option<IpAddr>,
  pub dns: DNSConfig,
  pub reserved_ports: Vec<Port>,
  pub dynamic_ports: Vec<Port>,
  // NOTE: Deprecated fields omitted
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequestedDevice {
  pub name: String,
  pub count: Option<u64>,
  pub constraints: Vec<Constraint>,
  pub affinities: Vec<Affinity>,
}
