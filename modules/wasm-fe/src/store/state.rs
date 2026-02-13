use dioxus::prelude::Signal;

#[derive(Clone, Debug)]
pub struct AppState {
  pub auth: Signal<Option<UserData>>,
}

#[derive(Debug, Clone, Default)]
pub struct UserData {
  pub token: String,
  pub user_id: i64,
}
