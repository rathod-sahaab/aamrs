use crate::ui::{
    components::{modal::ModalState, ProjectsList},
    pages::NewProject,
};
use dioxus::prelude::*;
use dioxus_free_icons::icons::fi_icons::FiPlus;
#[component]
pub fn Sidebar() -> Element {
    let mut modal_state = consume_context::<Signal<ModalState>>();
    rsx!(
        div { class: "bg-base-200 h-full flex flex-col",
            button {
                class: "btn btn-accent m-8",
                onclick: move |_| { modal_state.write().open("Create new project".to_string(), NewProject) },
                dioxus_free_icons::Icon { icon: FiPlus }
                "Create a new project"
            }
            ProjectsList {}
        }
    )
}
