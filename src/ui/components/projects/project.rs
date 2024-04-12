use dioxus::prelude::*;
use dioxus_free_icons::icons::fi_icons::FiChevronDown;

use crate::ui::components::projects::project_contents::ProjectContents;

#[derive(Props, Clone, PartialEq)]
pub struct ProjectProps {
    name: String,
    location: String,
}

pub fn Project(project:ProjectProps) -> Element {
    // TODO: active
    rsx!(
        li { class: "flex flex-col p-4",
            span {
                class: "tooltip tooltip-right inline-flex space-between items-center rounded-sm",
                "data-tip": "{project.location}",
                "{project.name}"
                dioxus_free_icons::Icon { class: "inline", icon: FiChevronDown }
            }
            ProjectContents { location: &project.location.clone() }
        }
    )
}
