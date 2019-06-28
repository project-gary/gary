pub trait ClusterApi {}

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug)]
pub enum ResourceAction {
    Get = 0,
    Set,
    Update,
}

pub enum ResourceType {
    Nodes = 0,
}

pub trait ClusterApi {

}

pub struct ClusterApiRequest {
    fn ReciveClusterRequest
}

pub struct ClusterRequest {
    action: ResourceAction,
    r_type: ResourceType,
    args: String,
}
