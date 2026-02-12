use dioxus::prelude::*;

#[component]
pub fn PostCard(title: String, content: String) -> Element {
  rsx! {
    div {
      id: "post-card",
      span {
        id: "post-card-title",
        {title}
      }
      span {
        id: "post-card-content",
        {content}
      }
    }
  }
}
