use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Join,
    Remove,
    Gossip,
    Sync,
    Ping,
    Heartbeat,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message<'a> {
    pub target: &'a str,
    pub sender: &'a str,
    pub msg_type: MessageType,
    pub payload: Vec<String>, // Maybe change to something more JSON friendly
}
