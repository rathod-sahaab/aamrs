use dioxus::prelude::*;
use dioxus_free_icons::icons::io_icons::IoAddSharp;

use crate::{ui::components::ProjectsList, Route};

#[component]
pub fn Sidebar() -> Element {
    return rsx!(
        div { class: "bg-base-200 h-full flex flex-col",
            Link { to: Route::NewProject {}, class: "btn btn-primary m-8",
                dioxus_free_icons::Icon { icon: IoAddSharp }
                "Create a new project"
            }
            ProjectsList {}
        }
    );
}
