use crate::components::post_card::PostCard;
use dioxus::prelude::*;

#[component]
pub fn PostsList() -> Element {
  rsx! {
      div {
          id: "posts-list",
          PostCard { title: "Hello", content: "Kitty" },
          PostCard { title: "Hello", content: "Kitty" },
          PostCard { title: "Hello", content: "Kitty" },
          PostCard { title: "Hello", content: "Kitty" },
          PostCard { title: "Hello", content: "Kitty" },
      }
  }
}

// rsx! {
//     for user in users {
//         div {
//             key: "{user.id}",
//             "{user.name}"
//         }
//     }
// }
