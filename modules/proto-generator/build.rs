fn main() {
  println!("cargo:rerun-if-changed=.env");
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=src/config.toml");
}
