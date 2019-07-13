use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

pub fn manage_deployments(_: Sender<&str>, _: Receiver<&str>) {
    thread::sleep(Duration::from_secs(1));
    println!("boom");
    thread::sleep(Duration::from_secs(1));
}
