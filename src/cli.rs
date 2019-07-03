use crate::cluster_api;
use crate::cluster_management;
use crate::deployment_management;
use clap::{App, Arg};
use core::data::{DeploymentCommand, DeploymentReply};
use daemonize::Daemonize;
// use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
                .multiple(false)
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
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .takes_value(true)
                .multiple(true)
                .help("The initial node to try and reach to join the cluster"),
        )
        .get_matches();

    let mut node_hash: HashMap<String, DateTime<Utc>> = HashMap::new();

    let mut targets: Vec<String> = Vec::new();
    if matches.is_present("target") {
        let mut t = matches.values_of("target").unwrap();
        for f in t {
            node_hash.insert(String::from(f), Utc::now());
        }
    }

    let mutex = Mutex::new(node_hash);
    let cluster_nodes: Arc<Mutex<HashMap<String, DateTime<Utc>>>> = Arc::new(mutex);

    //TODO: use config file if exists for vars
    if matches.is_present("daemon") {
        println!("Running as daemon");
        let daemonize = Daemonize::new()
            .pid_file("/var/run/gary.pid")
            .chown_pid_file(true)
            .working_directory("/tmp")
            .user("root")
            .group("daemon");
        match daemonize.start() {
            Ok(_) => run(cluster_nodes),
            Err(e) => println!("Should log failure to become daemon. error: {}", e),
        };
    } else {
        run(cluster_nodes);
    }
}

fn run(targets: Arc<Mutex<HashMap<String, DateTime<Utc>>>>) {
    println!("Starting server");

    // cluster consts - need to be CLI args
    const NODEHOSTNAME: &str = "nodehostname8675309";
    // const NODELISTENERPORT: u16 = 5555;

    //create thread channels
    let (tx_cm, rx_cm): (mpsc::Sender<&str>, mpsc::Receiver<&str>) = mpsc::channel();
    // commands sent to deployment manager
    let (tx_dmc, rx_dmc): (
        mpsc::Sender<DeploymentCommand>,
        mpsc::Receiver<DeploymentCommand>,
    ) = mpsc::channel();
    // command results / replies from deployment manager
    let (tx_dmr, rx_dmr): (
        mpsc::Sender<DeploymentReply>,
        mpsc::Receiver<DeploymentReply>,
    ) = mpsc::channel();
    // Channel to main thread for debug
    let (tx_debug, rx_debug): (mpsc::Sender<&str>, mpsc::Receiver<&str>) = mpsc::channel();

    let cm_targets = targets.clone();
    let cm_tx_debug = tx_debug.clone();
    //spawn a new thread for cluster management
    thread::spawn(move || {
        println!("starting first node in the cluster");
        // cluster_management::start_node(tx_cm, rx_dm);  // Communicates with Deployment Manager

        cluster_management::start_node(cm_tx_debug, tx_dmc, rx_dmr, NODEHOSTNAME, cm_targets); // Communicates with Main Thread
    });

    let api_nodes = targets.clone();
    thread::spawn(move || {
        cluster_api::start(api_nodes);
    });

    // spawn deployment management thread
    let dm_tx_debug = tx_debug.clone();
    thread::spawn(move || {
        println!("starting deployment management");
        deployment_management::manage_deployments(tx_dmr, rx_dmc, dm_tx_debug);
    });

    loop {
        // Consider using mpsc::Receiver::poll()
        match rx_debug.recv() {
            Ok(i) => println!("Debug message: {}", i),
            Err(_) => {
                println!("Debug channel closed");
                break;
            }
        }
    }

    //TODO: kill the threads
}
