use crate::cluster_management;
use crate::deployment_management;
use crate::cluster_api;
use clap::{App, Arg};
use daemonize::Daemonize;
// use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::{Arc,Mutex};

pub fn cli() {
    let matches = App::new("Gary")
        .version("0.1.0")
        .author("Marek C. <countsmarek@gmail.com>")
        .about("Does awesome things! very awesome.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("daemon")
                .short("d")
                .long("daemon")
                .multiple(false)
                .help("Setting this flag will enable running as a daemon"),
        )
        .get_matches();

    if matches.is_present("daemon") {
        println!("Running as daemon");
        let daemonize = Daemonize::new()
            .pid_file("/var/run/gary.pid")
            .chown_pid_file(true)
            .working_directory("/tmp")
            .user("root")
            .group("daemon");
        match daemonize.start() {
            Ok(_) => run(),
            Err(e) => println!("Should log failure to become daemon. error: {}", e),
        };
    } else {
        run();
    }
}

fn run() {
    println!("Starting server");

    let mut node_hash: HashMap<String, DateTime<Utc>> = HashMap::new();
    node_hash.insert("192.168.1.342".to_string(), Utc::now());
    node_hash.insert("Bobby".to_string(), Utc::now());
    let mutex = Mutex::new(node_hash);
    let cluster_nodes: Arc<Mutex<HashMap<String, DateTime<Utc>>>> = Arc::new(mutex);

    // cluster consts - need to be CLI args
    const NODEHOSTNAME: &str = "nodehostname8675309";
    // const NODELISTENERPORT: u16 = 5555;

    //create thread channels
    let (tx_cm, rx_cm): (mpsc::Sender<&str>, mpsc::Receiver<&str>) = mpsc::channel();
    let (tx_dm, rx_dm): (mpsc::Sender<&str>, mpsc::Receiver<&str>) = mpsc::channel();
    // Channel to main thread for debug
    let (tx_mt, rx_mt): (mpsc::Sender<&str>, mpsc::Receiver<&str>) = mpsc::channel();

    //spawn a new thread for cluster management
    thread::spawn(move || {
        println!("starting first node in the cluster");
        // cluster_management::start_node(tx_cm, rx_dm);  // Communicates with Deployment Manager
        let init_neighbors: Vec<String> = vec![
            // TODO: testing - remove later
            "somenode1".to_string(),
            "somenode2".to_string(),
            "somenode3".to_string(),
            "somenode4".to_string(),
        ];
        cluster_management::start_node(tx_mt, rx_dm, NODEHOSTNAME, init_neighbors); // Communicates with Main Thread
    });

    let api_nodes = cluster_nodes.clone();
    thread::spawn(move || {
        cluster_api::start(api_nodes);
    });

    // run deployment management on this thread
    println!("starting deployment management");
    deployment_management::manage_deployments(tx_dm, rx_cm);

    loop {
        // Consider using mpsc::Receiver::poll()
        match rx_mt.recv() {
            Ok(i) => println!("got this from cluster mgmt: {}", i),
            Err(_) => {
                // println!("channel closed");
                // break;
            }
        }
    }
}
