use dioxus::prelude::*;

#[component]
pub fn PostCard(is_owner: bool, title: String, content: String) -> Element {
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
              info!("Home button")
            },
            "Edit",
          }
          button {
            id: "my-button",
            onclick: move |_| {
              info!("Home button")
            },
            "Delete",
          }
        }
      }
    }
  }
}
