fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../../proto/moderation.proto");

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .out_dir(std::env::var("OUT_DIR").unwrap())
        .compile(&["../../proto/moderation.proto"], &["../../proto"])?;

    Ok(())
}
