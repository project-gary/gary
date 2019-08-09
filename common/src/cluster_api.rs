use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug)]
pub enum ResourceAction {
    Get,
    Set,
    Update,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum ResourceType {
    Nodes,
}

pub trait ClusterApi {
    fn cluster_request(&self, req: ClusterRequest);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClusterRequest {
    pub action: ResourceAction,
    pub r_type: ResourceType,
    pub args: String,
}
