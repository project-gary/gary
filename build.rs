extern crate tower_grpc_build;

use std::io::{stderr, Write};
use std::process;

fn main() {
    // Build helloworld
    tower_grpc_build::Config::new()
        .enable_server(true)
        .enable_client(true)
        .build(&["proto/helloworld.proto"], &["proto/"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}
