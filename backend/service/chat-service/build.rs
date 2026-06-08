fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../../proto/moderation.proto");

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir(std::env::var("OUT_DIR").unwrap())
        .compile(&["../../proto/moderation.proto"], &["../../proto"])?;

    Ok(())
}
