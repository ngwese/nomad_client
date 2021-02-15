use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::rust::default_on_null;
use std::collections::HashMap;

use crate::resources::{NetworkResource, NodeDeviceResource};

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

/*
// Node is used to deserialize a node entry.
type Node struct {
    ID                    string
    Datacenter            string
    Name                  string
    HTTPAddr              string
    TLSEnabled            bool
    Attributes            map[string]string
    Resources             *Resources
    Reserved              *Resources
    NodeResources         *NodeResources
    ReservedResources     *NodeReservedResources
    Links                 map[string]string
    Meta                  map[string]string
    NodeClass             string
    Drain                 bool
    DrainStrategy         *DrainStrategy
    SchedulingEligibility string
    Status                string
    StatusDescription     string
    StatusUpdatedAt       int64
    Events                []*NodeEvent
    Drivers               map[string]*DriverInfo
    HostVolumes           map[string]*HostVolumeInfo
    CSIControllerPlugins  map[string]*CSIInfo
    CSINodePlugins        map[string]*CSIInfo
    CreateIndex           uint64
    ModifyIndex           uint64
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeResources {
    pub cpu: NodeCpuResources,
    pub memory: NodeMemoryResources,
    pub disk: NodeDiskResources,
    pub networks: Vec<NetworkResource>,
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
    pub segments: HashMap<String, String>,
}

/*
// CSINodeInfo is the fingerprinted data from a CSI Plugin that is specific to
// the Node API.
type CSINodeInfo struct {
    ID                      string
    MaxVolumes              int64
    AccessibleTopology      *CSITopology
    RequiresNodeStageVolume bool
}

// CSIControllerInfo is the fingerprinted data from a CSI Plugin that is specific to
// the Controller API.
type CSIControllerInfo struct {
    SupportsReadOnlyAttach           bool
    SupportsAttachDetach             bool
    SupportsListVolumes              bool
    SupportsListVolumesAttachedNodes bool
}

// CSIInfo is the current state of a single CSI Plugin. This is updated regularly
// as plugin health changes on the node.
type CSIInfo struct {
    PluginID                 string
    AllocID                  string
    Healthy                  bool
    HealthDescription        string
    UpdateTime               time.Time
    RequiresControllerPlugin bool
    RequiresTopologies       bool
    ControllerInfo           *CSIControllerInfo `json:",omitempty"`
    NodeInfo                 *CSINodeInfo       `json:",omitempty"`
}
*/

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
    pub details: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
    pub create_index: u64,
}

/*

// HostStats represents resource usage stats of the host running a Nomad client
type HostStats struct {
    Memory           *HostMemoryStats
    CPU              []*HostCPUStats
    DiskStats        []*HostDiskStats
    DeviceStats      []*DeviceGroupStats
    Uptime           uint64
    CPUTicksConsumed float64
}

type HostMemoryStats struct {
    Total     uint64
    Available uint64
    Used      uint64
    Free      uint64
}

type HostCPUStats struct {
    CPU    string
    User   float64
    System float64
    Idle   float64
}

type HostDiskStats struct {
    Device            string
    Mountpoint        string
    Size              uint64
    Used              uint64
    Available         uint64
    UsedPercent       float64
    InodesUsedPercent float64
}

// DeviceGroupStats contains statistics for each device of a particular
// device group, identified by the vendor, type and name of the device.
type DeviceGroupStats struct {
    Vendor string
    Type   string
    Name   string

    // InstanceStats is a mapping of each device ID to its statistics.
    InstanceStats map[string]*DeviceStats
}

// DeviceStats is the statistics for an individual device
type DeviceStats struct {
    // Summary exposes a single summary metric that should be the most
    // informative to users.
    Summary *StatValue

    // Stats contains the verbose statistics for the device.
    Stats *StatObject

    // Timestamp is the time the statistics were collected.
    Timestamp time.Time
}

// StatObject is a collection of statistics either exposed at the top
// level or via nested StatObjects.
type StatObject struct {
    // Nested is a mapping of object name to a nested stats object.
    Nested map[string]*StatObject

    // Attributes is a mapping of statistic name to its value.
    Attributes map[string]*StatValue
}

// StatValue exposes the values of a particular statistic. The value may be of
// type float, integer, string or boolean. Numeric types can be exposed as a
// single value or as a fraction.
type StatValue struct {
    // FloatNumeratorVal exposes a floating point value. If denominator is set
    // it is assumed to be a fractional value, otherwise it is a scalar.
    FloatNumeratorVal   *float64 `json:",omitempty"`
    FloatDenominatorVal *float64 `json:",omitempty"`

    // IntNumeratorVal exposes a int value. If denominator is set it is assumed
    // to be a fractional value, otherwise it is a scalar.
    IntNumeratorVal   *int64 `json:",omitempty"`
    IntDenominatorVal *int64 `json:",omitempty"`

    // StringVal exposes a string value. These are likely annotations.
    StringVal *string `json:",omitempty"`

    // BoolVal exposes a boolean statistic.
    BoolVal *bool `json:",omitempty"`

    // Unit gives the unit type: °F, %, MHz, MB, etc.
    Unit string `json:",omitempty"`

    // Desc provides a human readable description of the statistic.
    Desc string `json:",omitempty"`
}
*/

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