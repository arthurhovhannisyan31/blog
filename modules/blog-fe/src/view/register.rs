use dioxus::prelude::*;
use serde_json::json;

use crate::configs::route::Route;
use crate::infrastructure::state::{AppState, UserData};

#[component]
pub fn Register() -> Element {
  let navigator = use_navigator();
  let AppState {
    mut auth,
    client,
    mut storage,
  } = consume_context::<AppState>();
  let mut email = use_signal(|| "".to_string());
  let mut username = use_signal(|| "".to_string());
  let mut password = use_signal(|| "".to_string());
  let mut error = use_signal(|| false);

  let handle_register = move |_| async move {
    if username.read().is_empty()
      || email.read().is_empty()
      || password.read().is_empty()
    {
      error.set(true);
      return;
    }

    let auth_response = client()
      .register(
        username.read().to_string(),
        email.read().to_string(),
        password.read().to_string(),
      )
      .await;

    match auth_response {
      Ok(data) => {
        let user_data = UserData {
          token: format!("Bearer {}", data.token),
          user_id: data.user.user_id,
        };

        auth.set(Some(user_data.clone()));
        storage.set(json!(user_data).to_string());
        navigator.push(Route::Home {});
      }
      Err(err) => {
        error!(err = ?err);
      }
    }
  };

  rsx! {
    div {
      id: "register",
      class: "container",
      div {
        id: "register-content",
        if error() {
          span {
            "Make sure all fields are filled and try again",
          }
        }
        label {
          id: "form-label",
          "Email",
          input {
            id: "register-email",
            type: "email",
            placeholder: "Email",
            autocomplete: "email",
            value: "{email}",
            oninput: move |event| email.set(event.value())
          }
        }
        label {
          id: "form-label",
          "Username",
          input {
            id: "register-username",
            placeholder: "username",
            value: "{username}",
            oninput: move |event| username.set(event.value())
          }
        }
        label {
          id: "form-label",
          "Password",
          input {
            id: "register-password",
            placeholder: "password",
            autocomplete: "new-password",
            value: "{password}",
            oninput: move |event| password.set(event.value())
          }
        }
        button {
          id: "register-button",
          onclick: handle_register,
          "Register",
        }
        button {
          onclick: move |_| {
            email.set("".into());
            username.set("".into());
            password.set("".into());
          },
          "Clear"
        }
        button {
          onclick: move |_| {
            navigator.push(Route::Login {});
          },
          "Login with existing user"
        }
      }
    }
  }
}
