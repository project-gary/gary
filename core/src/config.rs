use crate defaults;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClusterConfig {
    pub gossip_config: GossipConfig,
    pub deployment_config: DeploymentManagerConfig,
    pub initial_targets: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GossipConfig {
    pub interval: i16,
    pub fanout: i8,
    pub port: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeploymentManagerConfig {
    pub port:i32,
}

impl ClusterConfig {
    pub NewDefault() -> &self {
        let targets = vec!{String::from(TARGET)};
        &ClusterConfig{
            gossip_config: GossipConfig{
                interval: GOSSIP_INTERVAL,
                fanout: GOSSIP_FANOUT,
                port: GOSSIP_PORT,
            },
            deployment_config: DeploymentManagerConfig {
                port: DEPLOYMENT_PORT,
            },
            initial_targets: targets
        }
    }
}