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
    let mut activeProject = use_signal(|| Some(projects.first()?.name.clone()));
    return rsx!(
        ul { class: "flex flex-col space-y-3",
            for project in projects {
                Project {
                    name: project.name.clone(),
                    location: project.location.clone(),
                    active: activeProject().is_some() && project.name.eq(&activeProject().unwrap()),
                    on_click: move |_| {
                        if activeProject().is_none() {
                            activeProject.set(Some(project.name.clone()))
                        } else {
                            activeProject.set(None)
                        }
                    }
                }
            }
        }
    );
}
