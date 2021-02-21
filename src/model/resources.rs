use serde::{Deserialize, Serialize};
use serde_with::rust::{default_on_error, default_on_null};
use std::collections::HashMap;
use std::net::IpAddr;

use super::constraint::Constraint;
use super::tasks::Affinity;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Resources {
    #[serde(rename = "CPU")]
    pub cpu: Option<i64>,
    #[serde(rename = "MemoryMB")]
    pub memory_mb: Option<i64>,
    #[serde(rename = "DiskMB")]
    pub disk_mb: Option<i64>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub networks: Vec<NetworkResource>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub devices: Vec<RequestedDevice>,
    // NOTE: Deprecated fields omitted
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Port {
    pub label: Option<String>,
    pub value: Option<i64>,
    pub to: Option<i64>,
    pub host_network: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DNSConfig {
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub servers: Vec<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub searches: Vec<String>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub options: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct NetworkResource {
    pub mode: Option<String>,
    pub device: Option<String>,
    #[serde(rename = "CIDR")]
    pub cidr: Option<String>,
    #[serde(rename = "IP", deserialize_with = "default_on_error::deserialize")]
    pub ip: Option<IpAddr>,
    #[serde(rename = "DNS")]
    pub dns: Option<DNSConfig>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub reserved_ports: Vec<Port>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub dynamic_ports: Vec<Port>,
    // NOTE: Deprecated fields omitted
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequestedDevice {
    pub name: String,
    pub count: Option<u64>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub constraints: Vec<Constraint>,
    #[serde(deserialize_with = "default_on_null::deserialize")]
    pub affinities: Vec<Affinity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeDeviceResource {
    pub vendor: String,
    #[serde(rename = "Type")]
    pub device_type: String,
    pub name: String,
    pub instances: Vec<NodeDevice>,
    pub attributes: HashMap<String, Attribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeDevice {
    #[serde(rename = "ID")]
    pub id: String,
    pub healthy: bool,
    pub health_description: String,
    pub locality: Option<NodeDeviceLocality>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDeviceLocality {
    #[serde(rename = "PciBusID")]
    pub pci_bus_id: String,
}

// Attribute is used to describe the value of an attribute, optionally
// specifying units.
//
// TODO: Switch this to an enum if possible.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attribute {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_val: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int_val: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_val: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool_val: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_resources_basic() {
        let js = r#"
         {
            "CPU": 500,
            "MemoryMB": 256,
            "DiskMB": null,
            "Networks": null,
            "Devices": null,
            "IOPS": null
        }"#;

        let r: Result<Resources, serde_json::Error> = serde_json::from_str(&js);
        if r.is_err() {
            println!("deserialize failed: {:?}", r);
        }
        let r = r.unwrap();
        assert_eq!(r.cpu.unwrap(), 500);
        assert_eq!(r.memory_mb.unwrap(), 256);
    }

    #[test]
    fn deserialize_resources_networks() {
        let js = r#"
        {
            "Networks": []
        }"#;
        let r: Resources = serde_json::from_str(&js).expect("deserialize failed");
        assert_eq!(r.networks.len(), 0);

        let js = r#"{
            "Networks": [
                {
                    "Mode": "",
                    "Device": "",
                    "CIDR": "",
                    "IP": "192.168.1.0",
                    "DNS": null,
                    "ReservedPorts": null,
                    "DynamicPorts": [
                        {
                            "Label": "db",
                            "Value": 0,
                            "To": 6379,
                            "HostNetwork": ""
                        }
                    ],
                    "MBits": null
                }
            ]
        }"#;
        let r: Resources = serde_json::from_str(&js).expect("deserialize failed");
        assert_eq!(r.networks.len(), 1);
        assert_eq!(r.networks[0].dynamic_ports.len(), 1);

        // Ensure empty string for IP translates to None
        let js = r#"{
            "Networks": [
                {
                    "IP": ""
                }
            ]
        }"#;
        let r: Resources = serde_json::from_str(&js).expect("deserialize failed");
        assert_eq!(r.networks.len(), 1);
        assert!(r.networks[0].ip.is_none());
    }

    #[test]
    fn deserialize_resource_devices() {
        let js = r#"
        {
            "Devices": []
        }"#;
        let r: Resources = serde_json::from_str(&js).expect("deserialize failed");
        assert_eq!(r.networks.len(), 0);

        let js = r#"{
            "Devices": [
                {
                    "Name": "nvidia/gpu",
                    "Count": 2,
                    "Constraints": null,
                    "Affinities": null
                }
            ]
        }"#;
        let r: Resources = serde_json::from_str(&js).expect("deserialize failed");
        assert_eq!(r.devices.len(), 1);
        assert_eq!(r.devices[0].name, "nvidia/gpu");
        assert_eq!(r.devices[0].count, Some(2));
        assert_eq!(r.devices[0].constraints.len(), 0);
        assert_eq!(r.devices[0].affinities.len(), 0);
    }
}
