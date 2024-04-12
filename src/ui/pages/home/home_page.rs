use dioxus::prelude::*;

use dioxus_free_icons::icons::io_icons::IoAddSharp;

use crate::Route;
use crate::ui::components::ProjectsList;

#[component]
pub fn Home() -> Element {
    rsx! {
        Link { to: Route::NewProject {}, class: "btn btn-primary",
            dioxus_free_icons::Icon { icon: IoAddSharp }
            "Create a new project"
        }
        ProjectsList {}
    }
}
