use std::sync::mpsc::{Sender, Receiver};

pub fn join_cluster(sender: Sender<i32>, receiver: Receiver<i32>) {
    loop {
        match receiver.recv() {
            Ok(i) => println!("got int: {}", i),
            Err(_) => println!("channel closed"),
        }
    }
}