use dioxus::prelude::*;
use crate::{
    resources::aamrs_state::AamrsProject, ui::components::projects::project::Project,
    STATE,
};
#[component]
pub fn ProjectsList() -> Element {
    let projects = STATE
        .read()
        .projects
        .iter()
        .map(|project| AamrsProject {
            name: project.name.clone(),
            location: project.location.clone(),
        })
        .collect::<Vec<AamrsProject>>();
    return rsx!(
        ul { class: "flex flex-col space-y-3",
            for project in projects {
                Project { name: project.name.clone(), location: project.location.clone() }
            }
        }
    );
}
