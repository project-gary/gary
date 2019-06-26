use super::data::*;

/*
* Node to node private cluster communication
* */
pub trait ClusterCommunicator {
    /*
     * Cluster management
     * */
    fn join_cluster(s: &Self, info: MachineInfo);
    fn gossip(&self, gossip: Vec<MachineInfo>);
    fn heartbeat(&self, peer: &str);
    fn sync_request(&self, peer: &str);
}

/* not sure if I like these could be part of the same trait*/
pub trait ClusterCommunicationReceiver {
    fn heartbeat_response(peer: &str);
    fn sync_response(info: MachineInfo);
}

/*
* Node to node private deployment communication00
* */
pub trait DeploymentCommunicator{
    /*
     * Deployment management
     * */
    fn request_run_deployment(deployment: Deployment);
}

/* not sure if I like these */
pub trait DeploymentCommunicationReceiver {

}

/*
* public api, aka, backend of cli tool and maybe web ui
* */
pub trait ClusterApi {
    fn my_fn();
}
