use std::path::PathBuf;

use crate::ui::components::projects::dir_contents::Directory;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::fi_icons::FiChevronDown, Icon};
#[derive(Props, Clone, PartialEq)]
pub struct ProjectProps {
    name: String,
    location: String,
}
pub fn Project(project: ProjectProps) -> Element {
    let project_location = use_signal(|| project.location);
    let mut is_expanded = use_signal(|| false);

    rsx!(
        li {
            class: "flex flex-col pl-4 pr-2 group relative",
            div {
                class: "space-x-2",
                onclick: move |event| {
                    event.stop_propagation();
                    is_expanded.set(!is_expanded());
                },
                Icon {
                    class: "inline",
                    icon: FiChevronDown,
                }
                "{project.name}",
            }
            Directory {
                directory: PathBuf::from(project_location()),
                name: project.name.clone(),
                render_root: false,
                is_expanded,
            }
        }
    )
}
