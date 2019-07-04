use core::comm::ClusterCommunicator;
use core::network::{Message, MessageType};
use std::sync::mpsc::Sender;

use std::collections::HashMap;

use chrono::{DateTime, Utc};

use rand::Rng;
use serde_cbor;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// #[derive(Serialize, Deserialize, Debug)]  // Can't serialize 'sender'
pub struct ZmqNode {
    host_addr: String, // Unique ID
    gossip_fanout: u8, // Adjacent nodes updated each gossip cycle
    // node_comm_port: u16,                        // Node communication port
    node_comm_ctx: zmq::Context, // zmq context - ToDo: make generic for other comm libs
    _main_thread_sender: Sender<&'static str>, // Sender to main thread channel
    pub adjacent: Arc<Mutex<HashMap<String, DateTime<Utc>>>>, // Contains vector of ids to minimize storage
    delinquent: HashMap<String, DateTime<Utc>>, // Format is (host_addr, time_reported)
    removed: HashMap<String, DateTime<Utc>>,    // Format is (host_addr, time_reported)
}

impl ClusterCommunicator for ZmqNode {
    fn send_message(&self, target: &str, msg: &Message) -> bool {
        const TIMEOUTPERIOD: i32 = 1000; // timeout in milliseconds

        let serialized_msg = serde_cbor::to_vec(msg).unwrap();
        let requester = self.node_comm_ctx.socket(zmq::REQ).unwrap();
        requester.set_sndtimeo(TIMEOUTPERIOD).unwrap();
        requester.set_rcvtimeo(TIMEOUTPERIOD).unwrap();
        let target_addr = format!("tcp://{}:5555", target);
        assert!(requester.connect(&target_addr).is_ok());
        requester.send(&serialized_msg, 0).unwrap();
        let mut ack: String = "".to_string();
        match requester.recv_string(0) {
            Ok(v) => ack = v.unwrap(),
            Err(e) => println!("Error sending message to {}: {:?}", target, e),
        }
        assert!(requester.disconnect(&target_addr).is_ok());
        if ack.len() > 0 {
            // TODO:  Check that ack == "ACK" ?  Or just message length > 0?
            return true;
        } else {
            return false;
        }
    }

    fn handle_message(&mut self, msg: &Message) {
        match &msg.msg_type {
            MessageType::Join => println!("Message Type Received: {:?}", &msg.msg_type),
            MessageType::Remove => println!("Message Type Received: {:?}", &msg.msg_type),
            MessageType::Gossip => {
                println!("Message Type Received: {:?}", &msg.msg_type);
                self.comm_recv_gossip(&msg.payload);
            }
            MessageType::Sync => println!("Message Type Received: {:?}", &msg.msg_type),
            MessageType::Ping => println!("Message Type Received: {:?}", &msg.msg_type),
            MessageType::Heartbeat => {
                println!("Message Type Received: {:?}", &msg.msg_type);
                // self.comm_recv_heartbeat();  // Currently handled in Node.run() by 'responder.send("ACK", 0).unwrap();'
            }
        }
    }

    fn comm_recv_gossip(&mut self, payload: &Vec<String>) {
        if payload.len() > 0 {
            for node_addr in payload {
                if let Ok(mut a) = self.adjacent.lock() {
                    if !a.contains_key(node_addr) {
                        a.insert(node_addr.to_string(), Utc::now());
                    }
                }
            }
        }
    }

    // fn comm_recv_heartbeat(&mut self) {
    // Currently handled in Node.run() by 'responder.send("ACK", 0).unwrap();'
    //     println!("received heartbeat");
    // }

    fn get_nghbr_sample(&self, a: &HashMap<String, DateTime<Utc>>) -> Vec<String> {
        let mut adj_node_sample: Vec<String> = Vec::new();
        if (a.len() as u8) <= self.gossip_fanout {
            // Not sure about the 'as' conversion
            adj_node_sample = a.keys().map(|x| x.clone()).collect::<Vec<String>>();
        } else {
            let adjacent_keys = a.keys().map(|x| x.clone()).collect::<Vec<String>>();
            let adjacent_keys_len = adjacent_keys.len();
            let mut rng = rand::thread_rng();
            while (adj_node_sample.len() as u8) < self.gossip_fanout {
                let rand_index = rng.gen_range(0, adjacent_keys_len);
                if !adj_node_sample.contains(&adjacent_keys[rand_index]) {
                    adj_node_sample.push(adjacent_keys[rand_index].to_string());
                }
            }
        }

        adj_node_sample
    }

    fn update_neighbors(&mut self) {
        if let Ok(mut a) = self.adjacent.lock() {
            if a.len() > 0 {
                let nghbr_sample = self.get_nghbr_sample(&a);
                let adjacent_vec = a.keys().map(|x| x.clone()).collect::<Vec<String>>();
                //println!("Sent update for {:?}", nghbr_sample);
                for nghbr in nghbr_sample {
                    let msg = Message {
                        target: &nghbr,
                        sender: &self.host_addr,
                        msg_type: MessageType::Gossip,
                        payload: adjacent_vec.clone(), // ToDo:  Fix to use a ref instead - https://matklad.github.io/2018/05/04/encapsulating-lifetime-of-the-field.html
                    };
                    let result = self.send_message(&nghbr, &msg);
                    // println!("Send result: {}", result);
                    if !result {
                        if !self.delinquent.contains_key(&nghbr) {
                            self.delinquent.insert(nghbr.clone(), Utc::now());
                        }
                        a.remove(&nghbr);
                    }
                }
            }
        }
    }

    fn delinquent_node_check(&mut self) {
        const DELINQUENTPERIOD: u64 = 3600; // seconds
        let now = Utc::now();

        let allowed_duration_delinquent = Duration::new(DELINQUENTPERIOD.into(), 0);

        println!("Checking delinquent nodes again...");
        let delinquent_vec = self
            .delinquent
            .keys()
            .map(|x| x.clone())
            .collect::<Vec<String>>();
        for node in delinquent_vec {
            let msg = Message {
                target: &node,
                sender: &self.host_addr,
                msg_type: MessageType::Heartbeat,
                payload: Vec::new(), // ToDo:  Fix to use a ref instead - https://matklad.github.io/2018/05/04/encapsulating-lifetime-of-the-field.html
            };
            let result = self.send_message(&node, &msg);
            // If node responds to heartbeat, move back to adjacent node - implement heartbeat
            if result {
                if let Ok(mut a) = self.adjacent.lock() {
                    a.insert(node.clone(), Utc::now());
                    self.delinquent.remove(&node);
                }
            } else {
                let node_delinquent_time = self.delinquent.get(&node).unwrap().clone(); // TODO:  More clones, must eliminate...
                let time_difference = now
                    .signed_duration_since(node_delinquent_time)
                    .to_std()
                    .unwrap();

                // If node doesn not respond and timestamp does not exceed spec'd duration, continue
                // If timestamp exceeds some spec'd duration, remove node or add to a deleted_nodes field in Node
                if time_difference > allowed_duration_delinquent {
                    self.removed.insert(node.clone(), Utc::now());
                    self.delinquent.remove(&node);
                }
            }
        }
    }
}

impl ZmqNode {
    pub fn new(
        mt_sender: Sender<&'static str>,
        host_addr: &str,
        init_nodes: Arc<Mutex<HashMap<String, DateTime<Utc>>>>,
        // listener_port: u16,
    ) -> ZmqNode {
        ZmqNode {
            host_addr: host_addr.to_string(),
            gossip_fanout: 3,
            // node_comm_port: listener_port,
            node_comm_ctx: zmq::Context::new(),
            _main_thread_sender: mt_sender,
            adjacent: init_nodes,       //HashMap<&str, DateTime<UTC>>,
            delinquent: HashMap::new(), //HashMap<&str, DateTime>,
            removed: HashMap::new(),    //HashMap<&str, DateTime>,
        }
    }

    pub fn run(&mut self) {
        // Server setup
        const GOSSIPPERIOD: u64 = 2; // seconds
        const DELCHECKPERIOD: u64 = 30; // seconds

        let responder = self.node_comm_ctx.socket(zmq::REP).unwrap();
        assert!(responder.bind("tcp://*:5555").is_ok());

        // Gossip
        let allowed_duration_gossip = Duration::new(GOSSIPPERIOD.into(), 0);
        let mut gossip_period_start_time = Instant::now(); // TODO: Switch to using chrono for time everywhere

        // Delinquent node check
        let allowed_duration_del_check = Duration::new(DELCHECKPERIOD.into(), 0);
        let mut del_check_period_start_time = Instant::now();

        loop {
            // TODO:  Examine if this loop is expensive
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
                self.handle_message(&deserialized); // TODO:  Handle messages on green threads to prevent over-running gossip interval
            }

            // Check if heartbeat interval elapsed, send heartbeat/update message to peers
            if gossip_period_start_time.elapsed() > allowed_duration_gossip {
                // TODO:  Debug print statements, remove later
                if let Ok(a) = self.adjacent.lock() {
                    println!("Here's what my adjacent nodes are now: {:#?}", a);
                    println!(
                        "Here's what my delinquent nodes are now: {:#?}",
                        self.delinquent
                    );
                }
                self.update_neighbors(); // TODO:  Send messages on green threads to prevent over-running gossip interval
                gossip_period_start_time = Instant::now();
            }

            // Check if heartbeat interval elapsed, send heartbeat/update message to peers
            if del_check_period_start_time.elapsed() > allowed_duration_del_check {
                self.delinquent_node_check(); // TODO:  Send messages on green threads to prevent over-running gossip interval
                del_check_period_start_time = Instant::now();
            }
        }
    }

    // TO BE IMPLEMENTED
    // fn join()                // May not be needed - just start a node with known adjacent nodes
    // fn add_node()            // This may belong in garyctl - just send a gossip message with new node
}
