use core::cluster_api::*;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::{Arc,Mutex};

pub struct ZmqClusterApi {
    zmq_ctx: zmq::Context,
    cluster_nodes: Arc<Mutex<HashMap<String, DateTime<Utc>>>>,
}

impl ZmqClusterApi {
    pub fn new(nodes: Arc<Mutex<HashMap<String, DateTime<Utc>>>>) -> ZmqClusterApi {
        ZmqClusterApi{
            zmq_ctx: zmq::Context::new(),
            cluster_nodes: nodes.clone(),
        }
    }

    pub fn run(&self) {
        let responder = self.zmq_ctx.socket(zmq::REP).unwrap();
        println!("Here");
        assert!(responder.bind("tcp://127.0.0.1:5556").is_ok());
        println!("After Assert");
        loop { 
            if let Ok(msg) = responder.recv_bytes(0) {
                let data = self.cluster_nodes.lock().unwrap();
                let message = "marek".as_bytes();
                let mut v = "".to_string();
                for (key, value) in data.iter() {
                    v = format!("{}\n{}", v, &key.clone());
                }
                let _ = responder.send(v.as_bytes(), 0);

            }
        }
    }
}



impl ClusterApi for ZmqClusterApi {
    fn ClusterRequest(&self, req: ClusterRequest) {
                
    }
}