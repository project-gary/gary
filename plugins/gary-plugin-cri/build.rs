use anyhow::{Context, Result};
use std::{env, path::PathBuf};

const PROTO_FILE: &str = "src/kubernetes/cri/proto/api.proto";

fn main() -> Result<()> {
    tonic_build::configure()
        .out_dir("src/kubernetes/cri/api")
        .compile(
            &[PROTO_FILE],
            &[&PathBuf::from(PROTO_FILE)
                .parent()
                .context("no path parent")?
                .display()
                .to_string()],
        )
        .context("compile CRI protobuffers")?;

    Ok(())
}