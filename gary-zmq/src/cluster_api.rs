use chrono::{DateTime, Utc};
use core::cluster_api::*;
use core::defaults::*;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
pub struct ZmqClusterApi {
    zmq_ctx: zmq::Context,
    cluster_nodes: Arc<Mutex<HashMap<String, DateTime<Utc>>>>,
}

impl ZmqClusterApi {
    pub fn new(nodes: Arc<Mutex<HashMap<String, DateTime<Utc>>>>) -> ZmqClusterApi {
        ZmqClusterApi {
            zmq_ctx: zmq::Context::new(),
            cluster_nodes: nodes,
        }
    }

    pub fn run(&self) {
        let responder = self.zmq_ctx.socket(zmq::REP).unwrap();

        let connection = format!("tcp://{}:{}", TARGET, GOSSIP_PORT);
        assert!(responder.bind(&connection).is_ok());
        loop {
            if let Ok(_msg) = responder.recv_bytes(0) {
                let data = self.cluster_nodes.lock().unwrap();
                let mut v = "".to_string();
                for (key, _) in data.iter() {
                    v = format!("{}\n{}", v, &key.clone());
                }
                let _ = responder.send(v.as_bytes(), 0);
            }
        }
    }
}

impl ClusterApi for ZmqClusterApi {
    fn cluster_request(&self, _req: ClusterRequest) {}
}
