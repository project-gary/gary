use std::thread;
use std::time::Duration;
use std::sync::mpsc::{Sender, Receiver};

pub fn manage_deployments(sender: Sender<i32>, receiver: Receiver<i32>) {
    thread::sleep(Duration::from_secs(1));
    sender.send(32).unwrap();
    println!("boom");
    thread::sleep(Duration::from_secs(1));
}