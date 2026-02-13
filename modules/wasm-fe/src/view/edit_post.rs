use dioxus::prelude::*;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;

use crate::configs::route::Route;
use crate::infrastructure::model::{PostResponse, UpdatePostRequest};
use crate::infrastructure::state::AppState;

#[component]
pub fn EditPost(id: i64) -> Element {
  let navigator = use_navigator();
  let auth_data = consume_context::<AppState>().auth;
  let mut title = use_signal(|| "".to_string());
  let mut content = use_signal(|| "".to_string());
  let mut post_data = use_resource(move || get_post(id));
  let mut extra_content: Signal<Result<VNode, RenderError>> =
    use_signal(|| rsx! {});

  let handle_update = move |_| async move {
    let _ = update_post(
      auth_data().unwrap_or_default().token,
      id,
      title().to_string(),
      content().to_string(),
    )
    .await
    .unwrap();

    post_data.restart();
    navigator.push(Route::Home {});
  };

  use_effect(move || {
    match &*post_data.read() {
      Some(Ok(data)) => {
        title.set(data.title.to_string());
        content.set(data.content.to_string());

        extra_content.set(rsx! {});
      }
      Some(Err(e)) => {
        extra_content.set(rsx! {
          // Handle case for new post
          if id != 0 {
          "An error occured: {e}"
          }
        });
      }
      None => {
        extra_content.set(rsx! {
          "Loading ..."
        });
      }
    };
  });

  rsx! {
    div {
      id: "post-edit",
      div {
       "{id}"
      },
      {extra_content()}
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

async fn get_post(id: i64) -> anyhow::Result<PostResponse> {
  let client = Client::builder()
    .user_agent("User-Agent: wasm-fe")
    .build()?;
  let response = client
    .get(format!("http://localhost:8080/api/v0/posts/{id}"))
    .send()
    .await?;
  let post = response.json::<PostResponse>().await?;

  Ok(post)
}

async fn update_post(
  token: String,
  id: i64,
  title: String,
  content: String,
) -> anyhow::Result<PostResponse> {
  let client = Client::builder()
    .user_agent("User-Agent: wasm-fe")
    .build()?;
  let response = client
    .put(format!("http://localhost:8080/api/v1/posts/{id}"))
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
