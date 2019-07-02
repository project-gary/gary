use core::data::{DeploymentCommand, DeploymentReply, Deployment};
use std::result::Result;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

// data structures should go somewhere else.  But, for now...

pub fn manage_deployments(
    tx: Sender<DeploymentReply>,
    rx: Receiver<DeploymentCommand>,
    debug_tx: Sender<&str>,
) {
    // check to see if we have a command waiting to run

    /* if we received a command to start a new deployment:
      - determine the type
      - find a manager for deployments of that type
      - respond with the node id running it / error (a Result)
    */
    thread::sleep(Duration::from_secs(1));
    println!("boom");
    debug_tx
        .send("Started deployment manager thread")
        .unwrap_or_else(|e| panic!("Failed to send debug message: {}", e));

    let timeout = Duration::from_secs(15);
    loop {
        let msg = rx.recv_timeout(timeout);
        if msg.is_err() {
            // https://doc.rust-lang.org/std/sync/mpsc/enum.RecvTimeoutError.html
            // Timeout or Disconnected
            debug_tx
                .send("manager command receive timed out")
                .unwrap_or_else(|e| panic!("Failed to send debug message: {}", e));
        }else{
            match msg.unwrap() {
              DeploymentCommand::NewDeploy(d) => {
                create_deployment(d);
                let name = "";
                tx.send(DeploymentReply::NewDeploy(
                  name, Result::Ok(name)
                )).unwrap(); //TODO: handle error?
              },
            }
        }
    }
}

fn create_deployment(d:Deployment){
  println!("Deployment is {:#?}", d);
}
