//! Compiles Protocol Buffers into native Rust types.

use std::env;
use std::path::{Path, PathBuf};

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("protos");

    generate_protos(&root)?;

    Ok(())
}

/// Schema used for serializing writes
///
/// Creates:
///
/// - `influxdata.iox.kinesis.v1.rs`
fn generate_protos(root: &Path) -> Result<()> {
    let proto_files = vec![root.join("kinesis.proto")];

    // Tell cargo to recompile if any of these proto files are changed
    for proto_file in &proto_files {
        println!("cargo:rerun-if-changed={}", proto_file.display());
    }

    prost_build::Config::new().bytes(&[".influxdata"]).compile_protos(&proto_files, &[root.into()])?;

    Ok(())
}
