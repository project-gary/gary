extern crate clap;
extern crate daemonize;
extern crate log;

//default plugins;
extern crate gary_docker;

mod cli;
mod cluster_api;
mod cluster_management;
mod deployment_management;
mod runtime_plugin_manager;
fn main() {
    cli::cli();
}
