use gary_zmq::cluster_communication::ZmqNode;
// use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

pub fn start_node(
    sender: Sender<&'static str>,
    _receiver: Receiver<&str>,
    host_addr: &str,
    init_neighbors: Arc<Mutex<HashMap<String, DateTime<Utc>>>>,
    // node_listener_port: u16,
) {
    // loop {
    //     match receiver.recv() {
    //         Ok(i) => println!("got int: {}", i),
    //         Err(_) => {
    //             println!("channel closed");
    //             break;
    //         }
    //     }
    // }

    println!("Initial representation of a running Node");
    let mut myself = ZmqNode::new(sender, host_addr, init_neighbors); //, sender.to_owned());
    myself.run();
}
