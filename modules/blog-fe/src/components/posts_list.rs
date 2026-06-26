use dioxus::prelude::*;

use crate::components::post_card::PostCard;
use crate::infrastructure::state::AppState;

#[component]
pub fn PostsList() -> Element {
  let AppState {
    auth,
    client,
    storage: _,
  } = consume_context::<AppState>();
  let posts_resource =
    use_resource(move || async move { client().list_posts().await });

  let post_cards: Element = match &*posts_resource.read() {
    Some(Ok(post_list)) => {
      rsx! {
        for post in post_list.posts.iter() {
          PostCard {
            id: post.id,
            is_owner: auth().map(|data| data.user_id == post.author_id) == Some(true),
            title: post.title.clone(),
            content: post.content.clone(),
            resource: posts_resource,
          }
        }
      }
    }
    Some(Err(e)) => rsx!("Error {e}"),
    None => rsx!("Loading"),
  };

  rsx! {
      div {
          id: "posts-list",
          {post_cards}
      }
  }
}
