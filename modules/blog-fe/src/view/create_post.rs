use dioxus::prelude::*;

use crate::configs::route::Route;
use crate::infrastructure::state::AppState;

#[component]
pub fn CreatePost() -> Element {
  let navigator = use_navigator();
  let AppState {
    auth,
    client,
    storage: _,
  } = consume_context::<AppState>();
  let mut title = use_signal(|| "".to_string());
  let mut content = use_signal(|| "".to_string());

  let handle_update = move |_| async move {
    if let Err(err) = client()
      .create_post(
        auth().unwrap_or_default().token,
        title().to_string(),
        content().to_string(),
      )
      .await
    {
      error!(err = ?err);
    }

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
