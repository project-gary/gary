extern crate log;
extern crate daemonize;
extern crate clap;

mod cli;
mod cluster_management;
mod deployment_management;
fn main() {
    cli::cli()
}
