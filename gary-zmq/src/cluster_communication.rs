use core::comm::{ClusterCommunicator};
use core::data::MachineInfo;

struct ZmqClusterCommunicator {
    ctx: zmq::Context,
}

impl ClusterCommunicator for ZmqClusterCommunicator {

    fn join_cluster(s: &Self, info: MachineInfo) {

    }
    fn gossip(&self, gossip: Vec<MachineInfo>) {

    }
    fn heartbeat(&self, peer: &str) {

    }
    fn sync_request(&self, peer: &str) {
        
    }
}