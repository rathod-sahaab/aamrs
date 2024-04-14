use dioxus::prelude::*;
use dioxus_free_icons::icons::io_icons::IoAddSharp;

use crate::ui::{components::{modal::ModalState, ProjectsList}, pages::NewProject};

#[component]
pub fn Sidebar() -> Element {
    let mut modal_state = consume_context::<Signal<ModalState>>();
    return rsx!(
        div { class: "bg-base-200 h-full flex flex-col",
            button {
                class: "btn btn-accent m-8",
                onclick: move |_| { modal_state.write().open("Create new project".to_string(), NewProject) },
                dioxus_free_icons::Icon { icon: IoAddSharp }
                "Create a new project"
            }
            ProjectsList {}
        }
    );
}
