use gary_zmq::cluster_communication::ZmqNode;
//use core::data::{DeploymentCommand,DeploymentReply,};
use core::data::{*};
// use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

pub fn start_node(
    sender: Sender<&'static str>,
    exec_cmd_tx: Sender<DeploymentCommand>,
    exec_cmd_rx: Receiver<DeploymentReply>,
    host_addr: &str,
    init_neighbors: Arc<Mutex<HashMap<String, DateTime<Utc>>>>,
    // node_listener_port: u16,
) {
    // loop {
    //     match receiver.recv() {
    //         Ok(i) => println!("got int: {}", i),
    //         Err(_) => {
    //             println!("channel closed");
    //             break;
    //         }
    //     }
    // }

    // hard-code a deployment
    let sleep_deployment = Deployment {
      version: "0.1".to_string(),
      kind: DeploymentType::Process,
      metadata: Metadata {
        name: Some("sleep_demo".to_string()),
        labels: None,
      },
      spec: DeploymentSpec {
        replicas: 3,
        template: DeploymentTemplate {
          metadata: Metadata {
            name: None,
            labels: None,
          },
          spec: Spec::ProcessSpec(ProcessSpec {
            cmd: "/bin/sleep".to_string(),
            args: Some("inf".to_string()),
          }),
        }
      }

    };
    exec_cmd_tx.send( DeploymentCommand::NewDeploy(sleep_deployment) ).unwrap(); //TODO: debug
    // end hard-code

    println!("Initial representation of a running Node");
    let mut myself = ZmqNode::new(sender, host_addr, init_neighbors); //, sender.to_owned());
    myself.run();
}
