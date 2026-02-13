use dioxus::prelude::*;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;

use crate::configs::route::Route;
use crate::infrastructure::model::PostsListResponse;
use crate::infrastructure::state::AppState;

#[component]
pub fn PostCard(
  id: i64,
  is_owner: bool,
  title: String,
  content: String,
  refetch: Resource<anyhow::Result<PostsListResponse>>,
) -> Element {
  let navigator = use_navigator();
  let auth_data = consume_context::<AppState>().auth;

  let handle_delete = move |_| async move {
    let _ = delete_post(auth_data().unwrap_or_default().token, id)
      .await
      .expect("Failed to delete post");
    refetch.restart();
  };

  rsx! {
    div {
      id: "post-card",
      div {
        id: "post-card-content",
        span {
          id: "post-card-title",
          {title}
        }
        span {
          id: "post-card-content",
          {content}
        }
      }
      if is_owner {
        div {
          id: "post-card-controls",
          button {
            id: "my-button",
            onclick: move |_| {
              navigator.push(Route::EditPost {id});
            },
            "Edit",
          }
          button {
            id: "my-button",
            onclick: handle_delete,
            "Delete",
          }
        }
      }
    }
  }
}

async fn delete_post(token: String, id: i64) -> anyhow::Result<()> {
  let client = Client::builder()
    .user_agent("User-Agent: wasm-fe")
    .build()?;
  let _ = client
    .delete(format!("http://localhost:8080/api/v1/posts/{id}"))
    .header(AUTHORIZATION, token)
    .send()
    .await?;

  Ok(())
}
