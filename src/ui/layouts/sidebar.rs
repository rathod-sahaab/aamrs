use dioxus::prelude::*;
use dioxus_free_icons::icons::io_icons::IoAddSharp;

use crate::{ui::{components::{modal::ModalState, ProjectsList}, pages::NewProject}, Route};

#[component]
pub fn Sidebar() -> Element {
    let mut modal_state = consume_context::<Signal<ModalState>>();
    return rsx!(
        div { class: "bg-base-200 h-full flex flex-col",
            // Link { to: Route::NewProject {}, class: "btn btn-primary m-8",
            //     dioxus_free_icons::Icon { icon: IoAddSharp }
            //     "Create a new project"
            // }
            button {
                class: "btn btn-accent m-8",
                onclick: move |_| {
                    println!("Clicked CNP button");
                    modal_state.write().open("Create new project".to_string(), NewProject())
                },
                dioxus_free_icons::Icon { icon: IoAddSharp }
                "Create a new project"
            }
            ProjectsList {}
        }
    );
}
