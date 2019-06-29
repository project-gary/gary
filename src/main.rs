extern crate clap;
extern crate daemonize;
extern crate log;

mod cli;
mod cluster_api;
mod cluster_management;
mod deployment_management;
fn main() {
    cli::cli();
}
