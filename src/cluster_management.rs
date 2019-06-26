use std::sync::mpsc::{Receiver, Sender};

// #[macro_use]
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_cbor;
use serde_derive::{Deserialize, Serialize};
use std::time::{Duration, Instant};


pub fn join_cluster(sender: Sender<i32>, receiver: Receiver<i32>) {
    loop {
        match receiver.recv() {
            Ok(i) => println!("got int: {}", i),
            Err(_) => println!("channel closed"),
        }
    }
}


// Begin data dump by dorf:

#[derive(Serialize, Deserialize, Debug)]
struct Node {
    id: String,                                 // Unique ID
    host: String,                               // IP address or FQDN
    known_nodes: HashMap<String, String>,       // Format is (id, host)
    adjacent: HashMap<String, DateTime<Utc>>,   // Contains vector of ids to minimize storage
    delinquent: HashMap<String, DateTime<Utc>>, // Format is (id, time_reported)
}

impl Node {
    fn new(id: &str, host: &str) -> Node {
        Node {
            id: id.to_string(),
            host: host.to_string(),
            known_nodes: HashMap::new(), //HashMap::<String, String>::new(),
            adjacent: HashMap::new(),    //HashMap<&str, DateTime<UTC>>,
            delinquent: HashMap::new(),  //HashMap<&str, DateTime>,
        }
    }

    fn run(&self) {
        // Server setup
        let context = zmq::Context::new();
        let responder = context.socket(zmq::REP).unwrap();
        assert!(responder.bind("tcp://*:5555").is_ok());

        let allowed_duration = Duration::new(1, 0);
        let mut start_time = Instant::now();
        loop {
            if responder
                .poll(zmq::POLLIN, 10)
                .expect("client failed polling")
                > 0
            {
                let message = responder.recv_msg(0).unwrap();
                // ToDo: Incoming message should allow for different types of message
                // like "update", "join", "ping", "health", etc
                // let deserialized: HashMap<String, String> =
                //     serde_json::from_str(&message.as_str().unwrap()).unwrap();
                let deserialized: HashMap<String, String> =
                    serde_cbor::from_slice(&message).unwrap();
                println!("Received {:?}", deserialized);
                responder.send("", 0).unwrap();
            }
            // Check if heartbeat interval elapsed, send heartbeat/update message to peers
            if start_time.elapsed() > allowed_duration {
                self.send_update();
                start_time = Instant::now();
            }
        }
    }

    fn send_update(&self) {
        println!("Sent update message!");
        // select adjacent nodes - get_adj_sample()
        // pack nodes into some data structure
        // send message with message type
    }

    // TO BE IMPLEMENTED
    // fn get_adj_sample()
    // fn join()
    // fn add_node()
    // fn send_message()
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug)]
enum MessageType {
    Join = 0,
    Remove,
    Gossip,
    Sync,
    Ping,
    Health,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    target: String,
    sender: String,
    msg_type: MessageType,
    payload: Vec<String>, // Maybe change to something more JSON friendly
}

// fn main() {
//     println!("Initial representation of a running Node");
//     let myself = Node::new("myid", "myhostname");
//     myself.run();
//     // println!("{}", myself.id)
// }
