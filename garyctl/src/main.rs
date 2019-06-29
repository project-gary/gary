extern crate clap;
extern crate core;
use clap::{App, Arg, SubCommand, ArgMatches};
use std::sync::mpsc::{channel, Receiver, Sender};
use core::cluster_api::*;
use zmq::{Context,Message};

fn main() {
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
        .subcommand(SubCommand::with_name("get")
            .about("gets recourse types")
            .version("0.1")
            .author("Marek C. <countsmarek@gmail.com>")
                .subcommand(SubCommand::with_name("nodes")
                    .about("gets recourse types")
                    .version("0.1")
                    .author("Marek C. <countsmarek@gmail.com>")
                    .arg(Arg::with_name("target")
                        .short("t")
                        .long("target")
                        .takes_value(true)
                        .help("print debug information verbosely"))))
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .multiple(false)
                .takes_value(true)
                .help("Setting this flag will enable running as a daemon"),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("get") {
        if let Some(matches) = matches.subcommand_matches("nodes") {
                get("nodes".to_string(), matches)
        }
    }

    println!("Hello, world!");
}

fn get(resource_type: String, matches: &ArgMatches) {
    println!("getting nodes");
    
    if !matches.is_present("target") {
        println!("No target given. Garyctl config file not supported yet. Please provide a target" );
    }

    if let Some(target) = matches.value_of("target") {
        println!("Getting list of nodes from {}", target);
        let ctx = zmq::Context::new();
        let mut sock = ctx.socket(zmq::SocketType::REQ).unwrap();
        let _ = sock.connect("tcp://127.0.0.1:5556");
        let payload = ClusterRequest{
            action: ResourceAction::Get,
            r_type: ResourceType::Nodes,
            args: "Marek".to_string(),
        };
        let serialized_msg = serde_cbor::to_vec(&payload).unwrap();
        println!("-> {:?}", payload);
        let _ = sock.send(serialized_msg,0);
        if let Ok(msg) = sock.recv_msg(0) {
            println!("Got response {:?}", msg);
        }
    }

    
}
