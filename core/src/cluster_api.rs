use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    fn ClusterRequest(&self, req: ClusterRequest);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClusterRequest {
    pub action: ResourceAction,
    pub r_type: ResourceType,
    pub args: String,
}
