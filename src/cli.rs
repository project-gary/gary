extern crate clap;
use clap::{Arg, App};
extern crate daemonize;

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
            Err(e) => error!("{}", e),
        }
    } else {
        println!("running in foreground");
        run();
    }
}

fn run() {
    println!("Starting server");
}
