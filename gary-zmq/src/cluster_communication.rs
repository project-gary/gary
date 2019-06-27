use core::comm::ClusterCommunicator;
use core::network::*;
use std::sync::mpsc::{Receiver, Sender};

// #[macro_use]
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rand::Rng;
use serde_cbor;
use serde_derive::{Deserialize, Serialize};
use std::time::{Duration, Instant};

// #[derive(Serialize, Deserialize, Debug)]  // Can't serialize 'sender'
pub struct ZmqNode {
    id: String,                                 // Unique ID
    host: String,                               // IP address or FQDN
    gossip_fanout: u8,                          // Adjacent nodes updated each gossip cycle
    node_comm_port: u16,                        // Node communication port
    node_comm_ctx: zmq::Context, // zmq context - ToDo: make generic for other comm libs
    main_thread_sender: Sender<&'static str>, // Sender to main thread channel
    known_nodes: HashMap<String, String>, // Format is (id, host)
    adjacent: HashMap<String, DateTime<Utc>>, // Contains vector of ids to minimize storage
    delinquent: HashMap<String, DateTime<Utc>>, // Format is (id, time_reported)
}

impl ClusterCommunicator for ZmqNode {
    fn send_message(&self, msg: &Message) -> bool {
        let serialized_msg = serde_cbor::to_vec(msg).unwrap();
        let requester = self.node_comm_ctx.socket(zmq::REQ).unwrap();
        assert!(requester.connect("tcp://localhost:5555").is_ok());
        requester.send(&serialized_msg, 0).unwrap();
        let ack = requester.recv_string(0).unwrap().unwrap(); // ToDo:  So many unwraps... needs fixin'
        if ack.len() > 0 {
            return true;
        } else {
            return false;
        }
    }
}

impl ZmqNode {
    pub fn new(
        id: &str,
        host: &str,
        listener_port: u16,
        mt_sender: Sender<&'static str>,
    ) -> ZmqNode {
        ZmqNode {
            id: id.to_string(),
            host: host.to_string(),
            gossip_fanout: 3,
            node_comm_port: listener_port,
            node_comm_ctx: zmq::Context::new(),
            main_thread_sender: mt_sender,
            known_nodes: HashMap::new(), //HashMap::<String, String>::new(),
            adjacent: HashMap::new(),    //HashMap<&str, DateTime<UTC>>,
            delinquent: HashMap::new(),  //HashMap<&str, DateTime>,
        }
    }

    fn send_to_chan(&self, val: &'static str) {
        self.main_thread_sender.send(val).unwrap();
    }

    pub fn run(&mut self) {
        self.adjacent.insert("somenode1".to_string(), Utc::now()); // testing - remove later
        self.adjacent.insert("somenode2".to_string(), Utc::now()); // testing - remove later
        self.adjacent.insert("somenode3".to_string(), Utc::now()); // testing - remove later
        self.adjacent.insert("somenode4".to_string(), Utc::now()); // testing - remove later

        // Server setup
        // let context = zmq::Context::new();
        // let responder = context.socket(zmq::REP).unwrap();
        let responder = self.node_comm_ctx.socket(zmq::REP).unwrap();
        assert!(responder.bind("tcp://*:5555").is_ok());

        // let requester = context.socket(zmq::REQ).unwrap();
        // assert!(requester.connect("tcp://localhost:5555").is_ok());

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

                // deserialization examples:
                // let deserialized: HashMap<String, String> = serde_json::from_str(&message.as_str().unwrap()).unwrap();
                // let deserialized: HashMap<String, String> = serde_cbor::from_slice(&message).unwrap();

                let deserialized: Message = serde_cbor::from_slice(&message).unwrap();
                responder.send("ACK", 0).unwrap();
                self.handle_message(&deserialized);
            }
            // Check if heartbeat interval elapsed, send heartbeat/update message to peers
            if start_time.elapsed() > allowed_duration {
                self.update_neighbors();
                // self.send_to_chan("sdjkkhdsjkfh");
                start_time = Instant::now();
            }
        }
    }

    fn get_nghbr_sample(&self) -> Vec<String> {
        // println!("Sending update message!");
        // self.send_to_chan("Sending update message!");
        // let mut adj_node_sample: HashMap<String, DateTime<Utc>> = HashMap::new();
        let mut adj_node_sample: Vec<String> = Vec::new();

        if (self.adjacent.len() as u8) <= self.gossip_fanout {
            // Not sure about the 'as' conversion
            adj_node_sample = self
                .adjacent
                .keys()
                .map(|x| x.clone())
                .collect::<Vec<String>>();
        } else {
            let adjacent_keys = self
                .adjacent
                .keys()
                .map(|x| x.clone())
                .collect::<Vec<String>>();
            let adjacent_keys_len = adjacent_keys.len();
            let mut rng = rand::thread_rng();
            while (adj_node_sample.len() as u8) < self.gossip_fanout {
                let rand_index = rng.gen_range(0, adjacent_keys_len);
                if !adj_node_sample.contains(&adjacent_keys[rand_index]) {
                    adj_node_sample.push(adjacent_keys[rand_index].to_string());
                }
            }
        }
        return adj_node_sample;
    }

    fn handle_message(&self, msg: &Message) {
        // match &msg.msg_type {
        //     MessageType::Gossip => println!("Received payload {:?}", &msg.payload),
        // }
        // if &msg.msg_type == MessageType::Gossip {
        //     let payload = &msg.payload;
        //     println!("Received payload {:?}", payload);
        // }
        println!("Received payload {:?}", &msg.payload);
    }

    fn update_neighbors(&self) {
        if self.adjacent.len() > 0 {
            let nghbr_sample = self.get_nghbr_sample();
            let adjacent_vec = self
                .adjacent
                .keys()
                .map(|x| x.clone())
                .collect::<Vec<String>>();
            println!("Sent update for {:?}", nghbr_sample);
            for nghbr in nghbr_sample {
                let msg = Message {
                    target: nghbr,
                    sender: self.id.clone(), // ToDo:  Fix to use a ref instead
                    msg_type: MessageType::Gossip,
                    payload: adjacent_vec.clone(), // ToDo:  Fix to use a ref instead - https://matklad.github.io/2018/05/04/encapsulating-lifetime-of-the-field.html
                };

                // println!("Sent update for {}", nghbr);
                // Get hostname for nghbr
                // self.send_update(nghbr.hostname, nghbr_sample);
            }
        }

        // select adjacent nodes - get_adj_sample()
        // pack nodes into some data structure
        // send message containing adjacent nodes with message type
    }

    // TO BE IMPLEMENTED
    // fn receive_update() - check each nghbr in known nodes and update hostname, generate chrono Datetime and insert with node id into 'adjacent'
    // fn get_adj_sample()
    // fn join()
    // fn add_node()
}
