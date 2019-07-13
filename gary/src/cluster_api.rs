use gary_zmq::cluster_api::*;

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn start(m: Arc<Mutex<HashMap<String, DateTime<Utc>>>>) {
    let m = ZmqClusterApi::new(m);
    m.run();
    println!("Cluster Api Running");
}
