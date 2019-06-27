use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

pub fn join_cluster(
    sender: Sender<&'static str>,
    receiver: Receiver<&str>,
    node_id: &str,
    node_hostname: &str,
    node_listener_port: u16,
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
    //TODO: Need to inject a `ZmqNode` to use here
    //let mut myself = Node::new(node_id, node_hostname, node_listener_port, sender);  //, sender.to_owned());
    //myself.run();
}
