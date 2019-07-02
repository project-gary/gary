use core::data::{Deployment, DeploymentCommand, DeploymentReply, DeploymentType};
use std::result::Result;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use process::process_executor;

// data structures should go somewhere else.  But, for now...

pub fn manage_deployments(
    tx: Sender<DeploymentReply>,
    rx: Receiver<DeploymentCommand>,
    debug_tx: Sender<&str>,
) {
    /* if we received a command to start a new deployment:
      - determine the type
      - find a manager for deployments of that type
      - respond with the node id running it / error (a Result)
    */
    debug(&debug_tx, "Started deployment manager thread");

    let timeout = Duration::from_secs(15);
    loop {
        let msg = rx.recv_timeout(timeout);
        if msg.is_err() {
            // https://doc.rust-lang.org/std/sync/mpsc/enum.RecvTimeoutError.html
            // Timeout or Disconnected
            debug(&debug_tx, "manager command receive timed out");
        } else {
            match msg.unwrap() {
                // new deployment
                DeploymentCommand::NewDeploy(d) => {
                    let name = d.metadata.name.clone().unwrap_or("unknown".to_string());
                    create_deployment(&d);
                    tx.send(DeploymentReply::NewDeploy(
                        d.metadata.name.clone().unwrap(),
                        Result::Ok(name),
                    ))
                    .unwrap(); //TODO: handle error?
                }
                // other commands!
            }
        }
    }
}

// probably a better way, but for now...
fn debug<'debuggin>(tx: &Sender<&'debuggin str>, msg: &'debuggin str) {
    tx.send(msg)
        .unwrap_or_else(|e| panic!("Failed to send debug message: {}", e));
}

// TODO: check for errors :D
fn create_deployment(d: &Deployment) {
    let name = d.metadata.name.clone().unwrap_or("unknown".to_string());
    println!("Deployment '{}' definition: {:#?}", name, d);
    match d.kind {
        DeploymentType::Process => {
            println!("Deploying a process");
            process_executor::deploy(&d);
        }
        DeploymentType::Docker => {
            println!("Deploying a container");
        }
    }
}
