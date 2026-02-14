use crate::configs::error::AppError;
use crate::configs::route::Route;
use crate::infrastructure::model::PostsListResponse;
use crate::infrastructure::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn PostCard(
  id: i64,
  is_owner: bool,
  title: String,
  content: String,
  resource: Resource<Result<PostsListResponse, AppError>>,
) -> Element {
  let navigator = use_navigator();
  let AppState {
    auth,
    client,
    storage: _,
  } = consume_context::<AppState>();

  let handle_delete = move |_| async move {
    if let Err(err) = client()
      .delete_post(auth().unwrap_or_default().token, id)
      .await
    {
      error!(err = ?err);
    }
    resource.restart();
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
