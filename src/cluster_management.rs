use gary_zmq::cluster_communication::ZmqNode;
// use std::collections::HashMap;
use chrono::Utc;
use std::sync::mpsc::{Receiver, Sender};

pub fn start_node(
    sender: Sender<&'static str>,
    receiver: Receiver<&str>,
    host_addr: &str,
    init_neighbors: Vec<String>,
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
    let mut myself = ZmqNode::new(sender, host_addr); //, sender.to_owned());
    if init_neighbors.len() > 0 {
        for node_addr in init_neighbors {
            myself.adjacent.insert(node_addr.to_string(), Utc::now());
        }
    }
    myself.run();
}
