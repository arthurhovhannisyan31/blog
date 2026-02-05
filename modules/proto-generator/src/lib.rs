pub mod blog {
  tonic::include_proto!("blog");

  pub const FILE_DESCRIPTOR: &[u8] =
    tonic::include_file_descriptor_set!("./proto_descriptor");
}
