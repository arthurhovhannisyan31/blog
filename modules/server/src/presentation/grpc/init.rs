use std::net::SocketAddr;
use std::sync::Arc;

use proto_generator::blog::{
  FILE_DESCRIPTOR,
  grpc_blog_protected_service_server::GrpcBlogProtectedServiceServer,
  grpc_blog_public_service_server::GrpcBlogPublicServiceServer,
};
use tonic::transport::{Error, Server};
use tonic_middleware::InterceptorFor;
use tonic_reflection::server::Builder;

use crate::application::{
  auth_service::AuthService, blog_service::BlogService,
};
use crate::data::{
  post_repository::PostgresPostRepository,
  user_repository::PostgresUserRepository,
};
use crate::infrastructure::{config::AppConfig, jwt::JwtService};
use crate::presentation::grpc::auth::{
  AuthInterceptor, AuthValidationServiceImpl,
};
use crate::presentation::grpc::server::{
  GrpcBlogProtectedServiceImpl, GrpcBlogPublicServiceImpl,
};

pub fn init_grpc_server(
  auth_service: AuthService<PostgresUserRepository>,
  blog_service: BlogService<PostgresPostRepository>,
  jwt_service: Arc<JwtService>,
  config: &AppConfig,
) -> impl Future<Output = Result<(), Error>> {
  let auth_interceptor = AuthInterceptor {
    auth_service: AuthValidationServiceImpl::new(
      auth_service.clone(),
      jwt_service.clone(),
    ),
  };

  let grpc_public_service =
    GrpcBlogPublicServiceImpl::new(auth_service.clone(), blog_service.clone());
  let grpc_protected_service = GrpcBlogProtectedServiceImpl::new(
    auth_service.clone(),
    blog_service.clone(),
  );

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
    .add_service(GrpcBlogPublicServiceServer::new(grpc_public_service))
    .add_service(InterceptorFor::new(
      GrpcBlogProtectedServiceServer::new(grpc_protected_service),
      auth_interceptor,
    ))
    .serve(grpc_addr);

  grpc_server
}
