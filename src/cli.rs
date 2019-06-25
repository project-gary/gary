use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use clap::{Arg, App};
use crate::deployment_management;
use crate::cluster_management;

use daemonize::{Daemonize};

pub fn cli() {
    let matches = App::new("Gary")
                    .version("0.1.0")
                    .author("Marek C. <countsmarek@gmail.com>")
                    .about("Does awesome things! very awesome.")
                    .arg(Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .help("Sets a custom config file")
                        .takes_value(true))
                    .arg(Arg::with_name("v")
                        .short("v")
                        .multiple(true)
                        .help("Sets the level of verbosity"))
                    .arg(Arg::with_name("daemon")
                        .short("d")
                        .long("daemon")
                        .multiple(false)
                        .help("Setting this flag will enable running as a daemon"))
                    .get_matches();

    if matches.is_present("daemon") {

        println!("Running as daemon");
        let daemonize = Daemonize::new().pid_file("/var/run/gary.pid")
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

    //create thread channels
    let (tx_cm, rx_cm): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let (tx_dm, rx_dm): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    println!("Starting server");

    //spawn a new thread for cluster management
    thread::spawn(move || {
        println!("joining cluster");
        cluster_management::join_cluster(tx_cm,rx_dm);
    });

    //run deployment management on this thread
    println!("starting deployment management");
    deployment_management::manage_deployments(tx_dm,rx_cm);

}
