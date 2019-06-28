use serde::{Deserialize, Serialize};
use serde_cbor;

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Join,
    Remove,
    Gossip,
    Sync,
    Ping,
    Health,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub target: String,
    pub sender: String,
    pub msg_type: MessageType,
    pub payload: Vec<String>, // Maybe change to something more JSON friendly
}
