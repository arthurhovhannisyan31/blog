pub mod client;
pub mod error;
pub mod grpc_client;
pub mod http_client;

use grpc_client::GrpcBlogClient;

pub enum Transport {
  Http(String),
  Grpc(String),
}

// TODO create a single impl for both http and grpc
pub struct BlogClient {
  transport: Transport,
  http_client: Option<GrpcBlogClient>, // TODO Replace with http
  grpc_client: Option<GrpcBlogClient>,
  token: Option<String>,
}
pub struct User {}

// impl BlogCLient
// new(transport) — создание клиента с инициализацией HTTP или gRPC-соединения.
// set_token(token) и get_token() — управление JWT-токеном.
// register(username, email, password) — регистрация через HTTP или gRPC, сохранение токена.
// login(username, password) — вход через HTTP или gRPC, сохранение токена.
// create_post(title, content) — создание поста (требует токен).
// get_post(id) — получение поста.
// update_post(id, title, content) — обновление поста (требует токен).
// delete_post(id) — удаление поста (требует токен).
// list_posts(limit, offset) — список постов с пагинацией.
