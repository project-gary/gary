use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

pub fn manage_deployments(sender: Sender<&str>, receiver: Receiver<&str>) {
    thread::sleep(Duration::from_secs(1));
    sender.send("35").unwrap();
    println!("boom");
    thread::sleep(Duration::from_secs(1));
}
