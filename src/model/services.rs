use serde::{Deserialize, Serialize};
use serde_with::rust::{default_on_error, default_on_null};

use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;

use super::resources::Resources;
use super::serde_helpers::hashi_duration;
use super::tasks::LogConfig;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct CheckRestart {
    pub limit: Option<i64>,
    #[serde(with = "hashi_duration")]
    pub grace: Option<Duration>,
    pub ignore_warnings: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ServiceCheck {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "Type")]
    pub check_type: Option<String>,
    pub command: Option<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub args: Vec<String>,
    pub path: Option<String>,
    pub protocol: Option<String>,
    pub port_label: Option<String>,
    pub expose: bool,
    pub address_mode: Option<String>,
    #[serde(with = "hashi_duration")]
    pub interval: Option<Duration>,
    #[serde(with = "hashi_duration")]
    pub timeout: Option<Duration>,
    #[serde(rename = "TLSSkipVerify")]
    pub tls_skip_verify: bool,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub header: HashMap<String, String>,
    pub method: Option<String>,
    pub check_restart: Option<CheckRestart>,
    #[serde(rename = "GRPCService")]
    pub grpc_service: Option<String>,
    #[serde(rename = "GRPCUseTLS")]
    pub grpc_use_tls: bool,
    pub task_name: Option<String>,
    pub success_before_passing: Option<i64>,
    pub failures_before_critical: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Service {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub tags: Vec<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub canary_tags: Vec<String>,
    pub enable_tag_override: bool,
    pub port_label: Option<String>,
    pub address_mode: Option<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub checks: Vec<ServiceCheck>,
    pub check_restart: Option<CheckRestart>,
    pub connect: Option<ConsulConnect>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub meta: HashMap<String, String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub canary_meta: HashMap<String, String>,
    pub task_name: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConsulConnect {
    pub native: bool,
    pub gateway: Option<ConsulGateway>,
    pub sidecar_service: Option<ConsulSidecarService>,
    pub sidecar_task: Option<ConsulSidecarTask>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConsulGateway {
    pub proxy: Option<ConsulGatewayProxy>,
    pub ingress: Option<ConsulIngressConfigEntry>,
    pub terminating: Option<ConsulTerminatingConfigEntry>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConsulGatewayBindAddress {
    pub name: Option<String>,
    #[serde(deserialize_with = "default_on_error::deserialize")]
    pub address: Option<IpAddr>,
    pub port: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConsulGatewayProxy {
    #[serde(with = "hashi_duration")]
    pub connect_timeout: Option<Duration>,
    pub envoy_gateway_bind_tagged_address: bool,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub envoy_gateway_bind_addresses: HashMap<String, ConsulGatewayBindAddress>,
    pub envoy_gateway_no_default_bind: bool,
    #[serde(rename = "EnvoyDNSDiscoveryType")]
    pub envoy_dns_discovery_type: Option<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConsulGatewayTLSConfig {
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConsulSidecarService {
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub tags: Vec<String>,
    pub port: Option<String>,
    pub proxy: Option<ConsulProxy>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SidecarTask {
    pub path: Option<String>,
    pub driver: Option<String>,
    pub user: Option<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub config: HashMap<String, serde_json::Value>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub env: HashMap<String, String>,
    pub resources: Option<Resources>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub meta: HashMap<String, String>,
    #[serde(with = "hashi_duration")]
    pub kill_timeout: Option<Duration>,
    pub log_config: Option<LogConfig>,
    #[serde(with = "hashi_duration")]
    pub shutdown_delay: Option<Duration>,
    pub kill_signal: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConsulUpstream {
    pub destination_name: Option<String>,
    pub local_bind_port: Option<i64>,
    pub datacenter: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConsulExposeConfig {
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub path: Vec<ConsulExposePath>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConsulExposePath {
    pub path: Option<String>,
    pub protocol: Option<String>,
    pub local_path_port: Option<i64>,
    pub listener_port: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConsulSidecarTask {
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub tags: Vec<String>,
    pub port: Option<String>,
    pub proxy: Option<ConsulProxy>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConsulProxy {
    pub local_service_address: Option<String>,
    pub local_service_port: Option<i64>,
    pub expose_config: Option<ConsulExposeConfig>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub upstreams: Vec<ConsulUpstream>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConsulIngressService {
    pub name: Option<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub hosts: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConsulIngresListener {
    pub port: Option<i64>,
    pub protocol: Option<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub services: Vec<ConsulIngressService>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConsulIngressConfigEntry {
    pub tls: Option<ConsulGatewayTLSConfig>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub listeners: Vec<ConsulIngresListener>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConsulLinkedService {
    pub name: Option<String>,
    #[serde(rename = "CAFile")]
    pub ca_file: Option<String>,
    pub cert_file: Option<String>,
    pub key_file: Option<String>,
    #[serde(rename = "SNI")]
    pub sni: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConsulTerminatingConfigEntry {
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub services: Vec<ConsulLinkedService>,
}
