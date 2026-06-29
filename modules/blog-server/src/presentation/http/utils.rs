use actix_web::cookie::{Cookie, SameSite, time};

pub fn get_auth_cookie(token: &str) -> Cookie<'static> {
  Cookie::build("Authorization", format!("Bearer {token}"))
    .path("/")
    .http_only(true)
    .secure(true)
    .same_site(SameSite::Strict)
    .max_age(time::Duration::days(7))
    .finish()
}
