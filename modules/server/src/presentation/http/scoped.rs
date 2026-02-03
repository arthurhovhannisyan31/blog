use crate::presentation::http::{
  auth::{health, login, register},
  posts::{create_post, delete_post, get_post, get_posts, update_post},
};
use actix_web::{web, Scope};

pub fn public_scope() -> Scope {
  web::scope("/v0")
    .service(get_post)
    .service(get_posts)
    .service(health)
    .service(register)
    .service(login)
}

pub fn protected_scope() -> Scope {
  web::scope("/v1")
    .service(create_post)
    .service(update_post)
    .service(delete_post)
}
