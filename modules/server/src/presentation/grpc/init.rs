use std::net::SocketAddr;
use std::sync::Arc;

use proto_generator::blog::{
  FILE_DESCRIPTOR, grpc_blog_service_server::GrpcBlogServiceServer,
};
use tonic::transport::{Error, Server};
use tonic_reflection::server::Builder;

use crate::application::{
  auth_service::AuthService, blog_service::BlogService,
};
use crate::data::{
  post_repository::PostgresPostRepository,
  user_repository::PostgresUserRepository,
};
use crate::infrastructure::{config::AppConfig, jwt::JwtService};
use crate::presentation::grpc::server::GrpcBlogServiceImpl;

pub fn init_grpc_server(
  auth_service: AuthService<PostgresUserRepository>,
  blog_service: BlogService<PostgresPostRepository>,
  jwt_service: Arc<JwtService>,
  config: &AppConfig,
) -> impl Future<Output = Result<(), Error>> {
  let grpc_service =
    GrpcBlogServiceImpl::new(auth_service, blog_service, jwt_service);

  let grpc_reflection_service = Builder::configure()
    .register_encoded_file_descriptor_set(FILE_DESCRIPTOR)
    .build_v1()
    .expect("Failed building grpc reflection service");

  let grpc_addr_str = format!("{}:{}", config.host.as_str(), config.grpc_port);
  let grpc_addr: SocketAddr = grpc_addr_str
    .parse()
    .expect("Failed parsing grpc socket address");

  let grpc_server = Server::builder()
    .add_service(grpc_reflection_service)
    .add_service(GrpcBlogServiceServer::new(grpc_service))
    .serve(grpc_addr);

  grpc_server
}
