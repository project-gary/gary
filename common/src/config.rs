use crate::defaults::*;
use crate::yaml::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ClusterConfig {
    //pub version: f32,
    #[serde(rename = "gossipConfig")]
    pub gossip_config: GossipConfig,

    #[serde(rename = "deploymentConfig")]
    pub deployment_config: DeploymentManagerConfig,

    #[serde(rename = "initialTargets")]
    pub initial_targets: Vec<String>,

    #[serde(rename = "nodeInfo")]
    pub node_info: NodeInfo,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GossipConfig {
    pub interval: i16,
    pub fanout: i8,
    pub port: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DeploymentManagerConfig {
    pub port: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NodeInfo {
    #[serde(rename = "nodeName")]
    pub node_name: String,
}

impl ClusterConfig {
    pub fn new_default() -> Self {
        let targets = vec![String::from(TARGET)];
        ClusterConfig {
            gossip_config: GossipConfig {
                interval: GOSSIP_INTERVAL,
                fanout: GOSSIP_FANOUT,
                port: GOSSIP_PORT,
            },
            deployment_config: DeploymentManagerConfig {
                port: DEPLOYMENT_PORT,
            },
            initial_targets: targets,
            node_info: NodeInfo {
                node_name: whoami::hostname(),
            },
        }
    }

    //TODO: Remove all the `.unwrap();` calls
    pub fn get_config_or_default(path: Option<&str>) -> ClusterConfig {
        if let Some(path) = path {
            if let Ok(file) = fs::read_to_string(path) {
                if let Ok(input) = serde_yaml::to_value(&file) {
                    let mut result = serde_yaml::to_value(&ClusterConfig::new_default()).unwrap();
                    merge(&mut result, &input);
                    if let Ok(result) = serde_yaml::from_value(result) {
                        return result;
                    }
                }
            }
        }
        return ClusterConfig::new_default();
    }
}
