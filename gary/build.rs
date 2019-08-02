use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("Here: {} ", out_dir);
    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    Command::new("cargo")
        .args(&[
            "--manifest-path ../../plugin-runtime-containerd",
            "--out-dir plugins/",
        ])
        .status()
        .unwrap();
}
