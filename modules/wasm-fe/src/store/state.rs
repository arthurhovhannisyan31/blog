use dioxus::prelude::Signal;

#[derive(Clone, Debug)]
pub struct AppState {
  pub user: Signal<Option<UserData>>,
}

#[derive(Clone, Debug)]
pub struct UserData {
  pub name: String,
  pub id: i64,
  pub token: String,
}
