use dioxus::prelude::*;
use reqwest::Client;

use crate::components::post_card::PostCard;
use crate::infrastructure::model::PostsListResponse;
use crate::infrastructure::state::AppState;

#[component]
pub fn PostsList() -> Element {
  let auth_data = consume_context::<AppState>().auth;

  // TODO move value to context to mutate
  let posts_resource = use_resource(move || get_posts());

  let post_cards: Element = match &*posts_resource.read() {
    Some(Ok(post_list)) => {
      for post in post_list.posts.iter() {
        let key = format!("{} {}", post.title.clone(), post.content.clone());
      }

      rsx! {
        for post in post_list.posts.iter() {
          PostCard {
            id: post.id.clone(),
            is_owner: auth_data().and_then(|data| Some(data.user_id)) == Some(post.author_id.clone()),
            title: post.title.clone(),
            content: post.content.clone(),
            refetch: posts_resource,
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

async fn get_posts() -> anyhow::Result<PostsListResponse> {
  let client = Client::builder()
    .user_agent("User-Agent: wasm-fe")
    .build()?;
  let response = client
    .get("http://localhost:8080/api/v0/posts?limit=50&offset=0")
    .send()
    .await?;
  let posts_list = response.json::<PostsListResponse>().await?;

  Ok(posts_list)
}
