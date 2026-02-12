use crate::components::post_card::PostCard;
use crate::store::model::PostsListResponse;
use crate::store::state::AppState;
use dioxus::html::completions::CompleteWithBraces::title;
use dioxus::prelude::*;
use reqwest::Client;

#[component]
pub fn PostsList() -> Element {
  let user_data = consume_context::<AppState>().user;
  let auth = user_data().and_then(|data| Some(data.id)) == Some(0);

  let data = use_resource(move || get_posts());

  let post_cards: Element = match &*data.read() {
    Some(Ok(post_list)) => {
      rsx! {
        for post in post_list.posts.iter() {
          PostCard {
            is_owner: user_data().and_then(|data| Some(data.id)) == Some(post.author_id.clone()),
            title: post.title.clone(),
            content: post.content.clone(),
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
    .get("http://localhost:8080/api/v0/posts")
    .send()
    .await?;
  let posts_list = response.json::<PostsListResponse>().await?;

  Ok(posts_list)
}
