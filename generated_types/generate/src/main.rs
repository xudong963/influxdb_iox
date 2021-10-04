//! Compiles Protocol Buffers into native Rust types.

use std::path::Path;
use std::ffi::OsStr;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    // Ignores all args and finds relative paths based on PWD and the command

    // example: generated_types/generate/target/debug/generate
    let current_exe = std::env::current_exe()?;

    // walk up parent tree looking for generated types
    let mut generated_types = current_exe.clone();
    let needle = OsStr::new("generated_types");
    loop {
        if generated_types.file_name() == Some(&needle) {
            break
        }
        if !generated_types.pop() {
            panic!("Can not find 'generated_types' in the path: {:?}", current_exe);
        }
    }

    let root = generated_types.join("protos");
    let out_dir = generated_types.join("src").join("generated");

    generate_grpc_types(&root, &out_dir)?;

    Ok(())
}

/// Schema used with IOx specific gRPC requests
///
/// Creates:
///
/// - `com.github.influxdata.idpe.storage.read.rs`
/// - `influxdata.iox.catalog.v1.rs`
/// - `influxdata.iox.management.v1.rs`
/// - `influxdata.iox.write.v1.rs`
/// - `influxdata.platform.storage.rs`
fn generate_grpc_types(root: &Path, out_dir: &Path) -> Result<()> {
    let storage_path = root.join("influxdata/platform/storage");
    let idpe_path = root.join("com/github/influxdata/idpe/storage/read");
    let catalog_path = root.join("influxdata/iox/catalog/v1");
    let management_path = root.join("influxdata/iox/management/v1");
    let write_path = root.join("influxdata/iox/write/v1");

    let proto_files = vec![
        storage_path.join("test.proto"),
        storage_path.join("predicate.proto"),
        storage_path.join("storage_common.proto"),
        storage_path.join("service.proto"),
        storage_path.join("storage_common_idpe.proto"),
        idpe_path.join("source.proto"),
        catalog_path.join("catalog.proto"),
        catalog_path.join("parquet_metadata.proto"),
        catalog_path.join("predicate.proto"),
        management_path.join("database_rules.proto"),
        management_path.join("chunk.proto"),
        management_path.join("partition.proto"),
        management_path.join("service.proto"),
        management_path.join("shard.proto"),
        management_path.join("jobs.proto"),
        write_path.join("service.proto"),
        root.join("influxdata/pbdata/v1/influxdb_pb_data_protocol.proto"),
        root.join("grpc/health/v1/service.proto"),
        root.join("google/longrunning/operations.proto"),
        root.join("google/rpc/error_details.proto"),
        root.join("google/rpc/status.proto"),
    ];
    let mut config = prost_build::Config::new();
    config
        .out_dir(out_dir)
        .compile_well_known_types()
        .disable_comments(&[".google"])
        .extern_path(".google.protobuf", "::pbjson_types")
        .bytes(&[".influxdata.iox.catalog.v1.AddParquet.metadata"])
        .btree_map(&[
            ".influxdata.iox.catalog.v1.DatabaseCheckpoint.sequencer_numbers",
            ".influxdata.iox.catalog.v1.PartitionCheckpoint.sequencer_numbers",
        ]);

    let descriptor_path = out_dir.join("proto_descriptor.bin");
    tonic_build::configure()
        .out_dir(out_dir)
        .file_descriptor_set_path(&descriptor_path)
        .format(true)
        .compile_with_config(config, &proto_files, &[root.into()])?;

    let descriptor_set = std::fs::read(descriptor_path)?;

    pbjson_build::Builder::new()
        .out_dir(out_dir)
        .register_descriptors(&descriptor_set)?
        .build(&[".influxdata", ".google.longrunning", ".google.rpc"])?;

    println!("Protobuf files written to: {:?}", out_dir);

    Ok(())
}
