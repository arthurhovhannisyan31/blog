use dioxus::prelude::*;

#[component]
pub fn PostEdit(id: i64) -> Element {
  rsx! {
    div {
      id: "post-edit",
      div {
       "{id}"
      },
      label {
        id: "form-label",
        "Title",
        input {
          id: "post-edit-title",
        }
      }
      label {
        id: "form-label",
        "Content",
        textarea {
          id: "post-edit-content",
        }
      }
      div {
        id: "post-edit-controls",
        button {
          id: "post-edit-save",
          "Save",
        }
        button {
          id: "post-edit-discard",
          "Discard",
        }
      }
    }
  }
}
