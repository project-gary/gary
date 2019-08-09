use super::data::*;
use super::network::*;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/*
* Node to node private cluster communication
* */
pub trait ClusterCommunicator {
    /*
     * Cluster management
     * */
    fn send_message(&self, target: &str, msg: &Message) -> bool;
    fn handle_message(&mut self, msg: &Message);
    fn get_nghbr_sample(&self, a: &HashMap<String, DateTime<Utc>>) -> Vec<String>;
    fn comm_recv_gossip(&mut self, payload: &Vec<String>);
    // fn comm_recv_heartbeat(&mut self);  // Currently handled in Node.run() by 'responder.send("ACK", 0).unwrap();'
    fn update_neighbors(&mut self);
    fn delinquent_node_check(&mut self);
}

/* not sure if I like these could be part of the same trait*/
pub trait ClusterCommunicationReceiver {
    fn heartbeat_response(peer: &str);
    fn sync_response(info: MachineInfo);
}

/*
* Node to node private deployment communication00
* */
pub trait DeploymentCommunicator {
    /*
     * Deployment management
     * */
    fn request_run_deployment(deployment: Deployment);
}

/* not sure if I like these */
pub trait DeploymentCommunicationReceiver {}
