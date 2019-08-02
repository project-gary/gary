use crate::cluster_api;
use crate::cluster_management;
use crate::deployment_management;
use crate::runtime_plugin_manager;
use clap::{App, Arg};
use core::config::*;
use daemonize::Daemonize;
// use std::sync::mpsc::{Receiver, Sender};
use std::env;
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

    let mut runtime_plugin_manager = runtime_plugin_manager::RuntimePluginManager::new();

    let mut dockerBox = Box::from(gary_docker::ContainerdRuntimePlugin::new());
    runtime_plugin_manager.load_in_memory_plugin(dockerBox);

    let mut cur_dir = env::current_dir().unwrap();
    cur_dir.push("plugins");
    println!("{}", cur_dir.to_str().unwrap());
    runtime_plugin_manager.load_plugins_in_dir(String::from(cur_dir.to_str().unwrap()));

    runtime_plugin_manager.start_workload("na".to_string(), "docker".to_string());

    let config = core::config::ClusterConfig::get_config_or_default(matches.value_of("config"));

    let mut node_hash: HashMap<String, DateTime<Utc>> = HashMap::new();

    if matches.is_present("target") {
        let t = matches.values_of("target").unwrap();
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
            Ok(_) => run(cluster_nodes, config),
            Err(e) => println!("Should log failure to become daemon. error: {}", e),
        };
    } else {
        run(cluster_nodes, config);
    }
}

fn run(targets: Arc<Mutex<HashMap<String, DateTime<Utc>>>>, init_config: ClusterConfig) {
    println!("Starting server");

    // cluster consts - need to be CLI args
    const NODEHOSTNAME: &str = "nodehostname8675309";
    // const NODELISTENERPORT: u16 = 5555;

    //create thread channels
    let (_tx_cm, rx_cm): (mpsc::Sender<&str>, mpsc::Receiver<&str>) = mpsc::channel();
    let (tx_dm, rx_dm): (mpsc::Sender<&str>, mpsc::Receiver<&str>) = mpsc::channel();
    // Channel to main thread for debug
    let (tx_mt, rx_mt): (mpsc::Sender<&str>, mpsc::Receiver<&str>) = mpsc::channel();

    let cm_targets = targets.clone();
    //spawn a new thread for cluster management
    thread::spawn(move || {
        println!("starting first node in the cluster");
        // cluster_management::start_node(tx_cm, rx_dm);  // Communicates with Deployment Manager

        cluster_management::start_node(tx_mt, rx_dm, NODEHOSTNAME, cm_targets); // Communicates with Main Thread
    });

    let api_nodes = targets.clone();
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
