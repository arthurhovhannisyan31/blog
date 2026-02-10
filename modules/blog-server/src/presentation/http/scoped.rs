use crate::presentation::http::{
  auth::{health, login, register},
  posts::{create_post, delete_post, get_post, list_posts, update_post},
};
use actix_web::{Scope, web};

use common::constants::http_scope;

pub fn public_scope() -> Scope {
  web::scope(http_scope::PUBLIC)
    .service(get_post)
    .service(list_posts)
    .service(health)
    .service(register)
    .service(login)
}

pub fn protected_scope() -> Scope {
  web::scope(http_scope::PROTECTED)
    .service(create_post)
    .service(update_post)
    .service(delete_post)
}
