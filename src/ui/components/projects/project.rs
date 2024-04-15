use dioxus::prelude::*;
use dioxus_free_icons::icons::{
    fi_icons::{FiChevronDown, FiChevronRight},
    io_icons::IoEllipsisVertical,
};
use crate::ui::components::projects::project_contents::ProjectContents;
#[derive(Props, Clone, PartialEq)]
pub struct ProjectProps {
    name: String,
    location: String,
    active: bool,
    on_click: EventHandler<MouseEvent>,
}
pub fn Project(project: ProjectProps) -> Element {
    rsx!(
        li {
            class: "flex flex-col px-4",
            onclick: move |event| project.on_click.call(event),
            div {
                class: "tooltip tooltip-right w-full text-left flex justify-between",
                "data-tip": "{project.location}",
                span {
                    if project.active {
                        dioxus_free_icons::Icon { class: "inline", icon: FiChevronDown }
                    } else {
                        dioxus_free_icons::Icon { class: "inline", icon: FiChevronRight }
                    }
                    span { "{project.name}" }
                }
                button { class: "btn btn-sm btn-ghost btn-circle",
                    dioxus_free_icons::Icon { class: "inline", icon: IoEllipsisVertical }
                }
            }
            div { class: format!("overflow-hidden {}", if project.active { "" } else { "max-h-0" }),
                ProjectContents { location: &project.location.clone() }
            }
        }
    )
}
