use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::rust::default_on_null;
use std::collections::HashMap;

use crate::resources::{NetworkResource, NodeDeviceResource, Resources};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeStatus {
    Initializing,
    Ready,
    Down,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeScheduling {
    Eligible,
    Ineligible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodePurgeResponse {
    #[serde(rename = "EvalIDs")]
    pub eval_ids: Vec<String>,
    pub eval_create_index: u64,
    pub node_modify_index: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DriverInfo {
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub attributes: HashMap<String, String>,
    pub detected: bool,
    pub healthy: bool,
    pub health_description: String,
    pub update_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostVolumeInfo {
    pub path: String,
    pub read_only: bool,
}

// Node is used to deserialize a node entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Node {
    #[serde(rename = "ID")]
    pub id: String,
    pub datacenter: String,
    pub name: String,
    #[serde(rename = "HTTPAddr")]
    pub http_addr: String,
    #[serde(rename = "TLSEnabled")]
    pub tls_enabled: bool,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub attributes: HashMap<String, String>,
    pub resources: Option<Resources>,
    pub reserved: Option<Resources>,
    pub node_resources: Option<NodeResources>,
    pub reserved_resources: Option<NodeReservedResources>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub links: HashMap<String, String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub meta: HashMap<String, String>,
    pub node_class: String,
    pub drain: bool,
    pub drain_strategy: Option<DrainStrategy>,
    pub scheduling_eligibility: String,
    pub status: String,
    pub status_description: String,
    pub status_updated_at: i64,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub events: Vec<NodeEvent>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub drivers: HashMap<String, DriverInfo>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub host_volumes: HashMap<String, HostVolumeInfo>,
    #[serde(
        rename = "CSIControllerPlugins",
        deserialize_with = "default_on_null::deserialize"
    )]
    pub csi_controller_plugins: HashMap<String, CSIInfo>,
    #[serde(
        rename = "CSINodePlugins",
        deserialize_with = "default_on_null::deserialize"
    )]
    pub csi_node_plugins: HashMap<String, CSIInfo>,
    pub create_index: u64,
    pub modify_index: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeResources {
    pub cpu: NodeCpuResources,
    pub memory: NodeMemoryResources,
    pub disk: NodeDiskResources,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub networks: Vec<NetworkResource>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub devices: Vec<NodeDeviceResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeCpuResources {
    pub cpu_shares: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeMemoryResources {
    #[serde(rename = "MemoryMB")]
    pub memory_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeDiskResources {
    #[serde(rename = "DiskMB")]
    pub disk_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeReservedResources {
    pub cpu: NodeReservedCpuResources,
    pub memory: NodeReservedMemoryResources,
    pub disk: NodeReservedDiskResources,
    pub networks: NodeReservedNetworkResources,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeReservedCpuResources {
    pub cpu_shares: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeReservedMemoryResources {
    #[serde(rename = "MemoryMB")]
    pub memory_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeReservedDiskResources {
    #[serde(rename = "DiskMB")]
    pub disk_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeReservedNetworkResources {
    pub reserved_host_ports: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CSITopology {
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub segments: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CSINodeInfo {
    #[serde(rename = "ID")]
    pub id: String,
    pub max_volumes: i64,
    pub accessible_topology: Option<CSITopology>,
    pub required_node_stage_volume: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CSIControllerInfo {
    pub supports_read_only_attach: bool,
    pub supports_attach_detach: bool,
    pub supports_list_volumes: bool,
    pub supports_list_volumes_attached_nodes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CSIInfo {
    #[serde(rename = "PluginID")]
    pub plugin_id: String,
    #[serde(rename = "AllocID")]
    pub alloc_id: String,
    pub healthy: bool,
    pub health_description: String,
    pub update_time: DateTime<Utc>,
    pub requires_controller_plugin: bool,
    pub requires_topologies: bool,
    pub controller_info: Option<CSIControllerInfo>,
    pub node_info: Option<CSINodeInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DrainStrategy {
    pub spec: DrainSpec,
    pub force_deadline: DateTime<Utc>,
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DrainSpec {
    pub deadline: DateTime<Utc>,
    pub ignore_system_jobs: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum NodeEventSubsystem {
    Drain,
    Driver,
    Heartbeat,
    Cluster,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeEvent {
    pub message: String,
    pub subsystem: NodeEventSubsystem,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub details: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
    pub create_index: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostStats {
    pub memory: Option<HostMemoryStats>,
    #[serde(rename = "CPU", deserialize_with = "default_on_null::deserialize")]
    pub cpu: Vec<HostCPUStats>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub disk_stats: Vec<HostDiskStats>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub device_stats: Vec<DeviceGroupStats>,
    pub uptime: u64,
    #[serde(rename = "CPUTicksConsumed")]
    pub cpu_ticks_consumed: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostMemoryStats {
    pub total: u64,
    pub available: u64,
    pub used: u64,
    pub free: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostCPUStats {
    #[serde(rename = "CPU")]
    pub cpu: String,
    pub user: f64,
    pub system: f64,
    pub idle: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostDiskStats {
    pub device: String,
    pub mountpoint: String,
    pub size: u64,
    pub used: u64,
    pub available: u64,
    pub used_percent: f64,
    pub inodes_used_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceGroupStats {
    pub vendor: String,
    #[serde(rename = "Type")]
    pub device_type: String,
    pub name: String,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub instance_stats: HashMap<String, DeviceStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceStats {
    pub summary: Option<StatValue>,
    pub stats: Option<StatObject>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatObject {
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub nested: HashMap<String, StatObject>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub attributes: HashMap<String, StatValue>,
}

// TODO: Switch this to an enum if possible.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_numerator_val: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_denominator_val: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub int_numerator_val: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int_denominator_val: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool_val: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// NodeListStub is a subset of information returned during node list operations.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct NodeListStub {
    pub address: String,
    #[serde(rename = "ID")]
    pub id: String,
    pub datacenter: String,
    pub name: String,
    pub node_class: String,
    pub version: String,
    pub drain: bool,
    pub scheduling_eligibility: String,
    pub status: String,
    pub status_description: String,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub drivers: HashMap<String, DriverInfo>,
    pub node_resources: Option<NodeResources>,
    pub reserved_resources: Option<NodeReservedResources>,
    pub create_index: u64,
    pub modify_index: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_node_list_stub() {
        let js = r#"
        {
            "Address": "127.0.0.1",
            "CreateIndex": 7,
            "Datacenter": "dc1",
            "Drain": false,
            "Drivers": {
              "docker": {
                "Attributes": {
                  "driver.docker.version": "20.10.2",
                  "driver.docker.bridge_ip": "172.17.0.1",
                  "driver.docker.runtimes": "io.containerd.runc.v2,io.containerd.runtime.v1.linux,runc",
                  "driver.docker.os_type": "linux",
                  "driver.docker": "true"
                },
                "Detected": true,
                "HealthDescription": "Healthy",
                "Healthy": true,
                "UpdateTime": "2021-02-14T16:55:47.21692-08:00"
              },
              "qemu": {
                "Attributes": {
                  "driver.qemu": "true",
                  "driver.qemu.version": "5.2.0"
                },
                "Detected": true,
                "HealthDescription": "Healthy",
                "Healthy": true,
                "UpdateTime": "2021-02-14T16:30:17.304615-08:00"
              },
              "java": {
                "Attributes": {
                  "driver.java.runtime": "Java(TM) SE Runtime Environment 18.3 (build 10.0.2+13)",
                  "driver.java.vm": "Java HotSpot(TM) 64-Bit Server VM 18.3 (build 10.0.2+13, mixed mode)",
                  "driver.java": "true",
                  "driver.java.version": "10.0.2"
                },
                "Detected": true,
                "HealthDescription": "Healthy",
                "Healthy": true,
                "UpdateTime": "2021-02-14T16:30:17.571206-08:00"
              },
              "exec": {
                "Attributes": null,
                "Detected": false,
                "HealthDescription": "exec driver unsupported on client OS",
                "Healthy": false,
                "UpdateTime": "2021-02-14T16:30:17.152305-08:00"
              },
              "mock_driver": {
                "Attributes": {
                  "driver.mock": "true"
                },
                "Detected": true,
                "HealthDescription": "Healthy",
                "Healthy": true,
                "UpdateTime": "2021-02-14T16:30:17.152557-08:00"
              },
              "raw_exec": {
                "Attributes": {
                  "driver.raw_exec": "true"
                },
                "Detected": true,
                "HealthDescription": "Healthy",
                "Healthy": true,
                "UpdateTime": "2021-02-14T16:30:17.152923-08:00"
              }
            },
            "HostVolumes": null,
            "ID": "47a4cc33-4bdc-a5f2-cdce-2a4017a58a72",
            "ModifyIndex": 36,
            "Name": "fakehostname.local",
            "NodeClass": "",
            "SchedulingEligibility": "eligible",
            "Status": "ready",
            "StatusDescription": "",
            "Version": "1.0.3"
          }
        "#;

        let _nls: NodeListStub = serde_json::from_str(js).expect("deserialize failed");
    }
}
