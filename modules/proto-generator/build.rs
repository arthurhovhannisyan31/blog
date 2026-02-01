use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=.env");
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=src/config.toml");

  let out_dir = PathBuf::from(env::var("OUT_DIR")?);

  tonic_prost_build::configure()
    .file_descriptor_set_path(out_dir.join("proto_descriptor.bin"))
    .build_server(true)
    .build_client(true)
    .compile_protos(&["proto/blog.proto"], &["proto/blog"])?;
  Ok(())
}
