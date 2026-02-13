use dioxus::prelude::*;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;

use crate::configs::route::Route;
use crate::store::model::{PostResponse, UpdatePostRequest};
use crate::store::state::AppState;

#[component]
pub fn CreatePost() -> Element {
  let navigator = use_navigator();
  let auth_data = consume_context::<AppState>().auth;
  let mut title = use_signal(|| "".to_string());
  let mut content = use_signal(|| "".to_string());

  let handle_update = move |_| async move {
    let _ = update_post(
      auth_data().unwrap_or_default().token,
      title().to_string(),
      content().to_string(),
    )
    .await
    .expect("Failed to update post");

    navigator.push(Route::Home {});
  };

  rsx! {
    div {
      id: "post-edit",
      label {
        id: "form-label",
        "Title",
        input {
          id: "post-edit-title",
          value: "{title}",
          onchange: move |event| title.set(event.value())
        }
      }
      label {
        id: "form-label",
        "Content",
        textarea {
          id: "post-edit-content",
          value: "{content}",
          onchange: move |event| content.set(event.value())
        }
      }
      div {
        id: "post-edit-controls",
        button {
          id: "post-edit-save",
          onclick: handle_update,
          "Save",
        }
        button {
          id: "post-edit-discard",
          onclick: move |_| {
            navigator.push(Route::Home {});
          },
          "Discard",
        }
      }
    }
  }
}

async fn update_post(
  token: String,
  title: String,
  content: String,
) -> anyhow::Result<PostResponse> {
  let client = Client::builder()
    .user_agent("User-Agent: wasm-fe")
    .build()?;
  let response = client
    .post("http://localhost:8080/api/v1/posts")
    .header(AUTHORIZATION, token)
    .json(&UpdatePostRequest {
      title: Some(title),
      content: Some(content),
    })
    .send()
    .await?;
  let post = response.json::<PostResponse>().await?;

  Ok(post)
}
