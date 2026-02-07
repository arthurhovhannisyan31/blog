pub const POST_STREAM_LIMIT: i64 = 10;
pub const POST_STREAM_OFFSET: i64 = 0;

pub mod http_scope {
  pub const PROTECTED: &str = "/v1";
  pub const PUBLIC: &str = "/v0";
}

pub mod http_route {
  pub const REGISTER: &str = "/auth/register";
  pub const LOGIN: &str = "/auth/login";
  pub const POST: &str = "/posts";
}
