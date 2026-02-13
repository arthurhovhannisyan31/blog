use dioxus::prelude::*;

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
  let AppState {
    auth,
    client,
    storage: _,
  } = consume_context::<AppState>();

  let handle_delete = move |_| async move {
    let _ = client()
      .delete_post(auth().unwrap_or_default().token, id)
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
