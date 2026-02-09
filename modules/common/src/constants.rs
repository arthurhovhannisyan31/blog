pub const QUERY_LIMIT: u64 = 10;
pub const QUERY_LIMIT_STEP: u64 = 10;
pub const QUERY_OFFSET: u64 = 0;

pub mod http_scope {
  pub const PROTECTED: &str = "/v1";
  pub const PUBLIC: &str = "/v0";
}

pub mod http_route {
  pub const REGISTER: &str = "/auth/register";
  pub const LOGIN: &str = "/auth/login";
  pub const POST: &str = "/posts";
}
