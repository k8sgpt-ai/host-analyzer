use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        //.out_dir("./src/proto") // if prefer to check-in generated artefacts
        .file_descriptor_set_path(out_dir.join("rust-analyzer.bin"))
        .compile(
            &["./schemas/protobuf/schema/v1/analyzer.proto"],
            &["./schemas/protobuf/schema/v1"], // location to search proto dependencies
        )?;

    Ok(())
}
