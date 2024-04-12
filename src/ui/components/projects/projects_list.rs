use dioxus::prelude::*;

use crate::{ui::components::projects::project::Project, STATE};

#[component]
pub fn ProjectsList() -> Element {
    // TODO: get from lazy static
    return rsx!(
        ul { class: "flex flex-col",
            for project in STATE.read().projects.iter() {
                Project { name: project.name.clone(), location: project.location.clone() }
            }
        }
    );
}
