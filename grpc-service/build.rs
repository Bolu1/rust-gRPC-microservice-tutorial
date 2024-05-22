use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("task_service_descriptor.bin"))
        .build_server(true)
        .compile(
            &["./protobuf/task_service.proto"],
            &["./protobuf/"]
        )
        .unwrap();

    Ok(())
}
