use dioxus::prelude::*;

use crate::configs::route::Route;
use crate::infrastructure::state::AppState;

#[component]
pub fn EditPost(id: i64) -> Element {
  let navigator = use_navigator();
  let AppState {
    auth,
    client,
    storage: _,
  } = consume_context::<AppState>();
  let mut title = use_signal(|| "".to_string());
  let mut content = use_signal(|| "".to_string());
  let mut post_data =
    use_resource(move || async move { client().get_post(id).await });
  let mut extra_content: Signal<Result<VNode, RenderError>> =
    use_signal(|| rsx! {});

  let handle_update = move |_| async move {
    if let Err(err) = client()
      .update_post(
        auth().unwrap_or_default().token,
        id,
        title().to_string(),
        content().to_string(),
      )
      .await
    {
      error!(err = ?err);
    }

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
