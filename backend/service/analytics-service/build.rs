use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let proto_file = PathBuf::from(&manifest_dir).join("../../proto/analytics.proto");
    let proto_dir = PathBuf::from(&manifest_dir).join("../../proto");

    println!("cargo:rerun-if-changed={}", proto_file.display());

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(&[proto_file], &[proto_dir])?;

    Ok(())
}
