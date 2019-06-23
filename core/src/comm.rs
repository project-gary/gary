use super::data::*;

pub trait ClusterCommunication {
    /*
     * Cluster management
     * */
    fn join_cluster(info: MachineInfo);
    fn gossip(gossip: Vec<MachineInfo>);
    fn sync_request();
    fn sync_response(info: MachineInfo);

    /*
     * Deployment management
     * */
    fn request_run_deployment(deployment: Deployment);
}

pub trait ClusterApi {
    fn my_fn();
}
