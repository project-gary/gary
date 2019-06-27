
use crate::cluster_management;
use crate::deployment_management;
use clap::{App, Arg};
use daemonize::Daemonize;
// use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;


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

    // cluster consts - need to be CLI args
    const NODEID: &str = "node8675309";
    const NODEHOSTNAME: &str = "nodehostname8675309";
    const NODELISTENERPORT: u16 = 5555;


    //create thread channels
    let (tx_cm, rx_cm): (mpsc::Sender<&str>, mpsc::Receiver<&str>) = mpsc::channel();
    let (tx_dm, rx_dm): (mpsc::Sender<&str>, mpsc::Receiver<&str>) = mpsc::channel();
    // Channel to main thread for debug
    let (tx_mt, rx_mt): (mpsc::Sender<&str>, mpsc::Receiver<&str>) = mpsc::channel();


    //spawn a new thread for cluster management
    thread::spawn(move || {
        println!("joining cluster");
        // cluster_management::join_cluster(tx_cm, rx_dm);  // Communicates with Deployment Manager
        cluster_management::join_cluster(tx_mt, rx_dm, NODEID, NODEHOSTNAME, NODELISTENERPORT); // Communicates with Main Thread
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
