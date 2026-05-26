fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../../proto/auth.proto");
    println!("cargo:rerun-if-changed=../../proto/stream_meta.proto");
    println!("cargo:rerun-if-changed=../../proto/errors.proto");

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir(std::env::var("OUT_DIR").unwrap())
        .compile(
            &[
                "../../proto/auth.proto",
                "../../proto/stream_meta.proto",
                "../../proto/errors.proto",
            ],
            &["../../proto"],
        )?;

    Ok(())
}
