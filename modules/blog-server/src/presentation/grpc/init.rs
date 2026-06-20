use std::sync::Arc;

use futures_util::TryFutureExt;
use proto_generator::blog::{
  FILE_DESCRIPTOR, blog_protected_service_server::BlogProtectedServiceServer,
  blog_public_service_server::BlogPublicServiceServer,
};
use tonic::transport::Server;
use tonic_middleware::InterceptorFor;
use tonic_reflection::server::Builder;

use crate::application::{
  auth_service::AuthService, blog_service::BlogService,
};
use crate::data::{
  post_repository::PostgresPostRepository,
  user_repository::PostgresUserRepository,
};
use crate::infrastructure::{
  config::AppConfig, error::ServerError, jwt::JwtService,
};
use crate::presentation::grpc::{
  auth::{AuthInterceptor, AuthValidationServiceImpl},
  service::{GrpcBlogProtectedServiceImpl, GrpcBlogPublicServiceImpl},
};

pub fn init_grpc_server(
  auth_service: Arc<AuthService<PostgresUserRepository>>,
  blog_service: Arc<BlogService<PostgresPostRepository>>,
  jwt_service: Arc<JwtService>,
  app_config: &AppConfig,
) -> impl Future<Output = Result<(), ServerError>> {
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
    .expect("Failed building gRPC reflection service");

  let mut server = Server::builder();

  #[cfg(feature = "tls")]
  {
    use crate::presentation::grpc::tls::build_grpc_tls_config;

    let tls_config = build_grpc_tls_config(&app_config)
      .unwrap_or_else(|err| panic!("Failed building gRPC TLS config: {}", err));

    let tls_server = server
      .tls_config(tls_config)
      .expect("Failed building gRPC TLS configuration");
    server = tls_server;
  }

  server
    .add_service(grpc_reflection_service)
    .add_service(BlogPublicServiceServer::new(grpc_public_service))
    .add_service(InterceptorFor::new(
      BlogProtectedServiceServer::new(grpc_protected_service),
      auth_interceptor,
    ))
    .serve(app_config.grpc_addr)
    .map_err(ServerError::from)
}
