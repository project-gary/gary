extern crate clap;
extern crate daemonize;
extern crate log;

mod cli;
mod cluster_management;
mod deployment_management;
mod cluster_api;
fn main() {
    cli::cli();
}
