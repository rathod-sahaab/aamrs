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
}
pub fn Project(project: ProjectProps) -> Element {
    let mut is_expanded = use_signal(|| false);

    rsx!(
        li {
            class: "flex flex-col pl-4 pr-2 group",
            onclick: move |_| is_expanded.set(!is_expanded()),
            div {
                class: "tooltip tooltip-right w-full text-left flex justify-between",
                "data-tip": "{project.location}",
                span {
                    if is_expanded() {
                        dioxus_free_icons::Icon { class: "inline", icon: FiChevronDown }
                    } else {
                        dioxus_free_icons::Icon { class: "inline", icon: FiChevronRight }
                    }
                    span { "{project.name}" }
                }
                button { class: "btn btn-sm btn-ghost btn-circle opacity-0 group-hover:opacity-100",
                    dioxus_free_icons::Icon { class: "inline", icon: IoEllipsisVertical }
                }
            }
            div { class: format!("overflow-hidden {}", if is_expanded() { "" } else { "max-h-0" }),
                ProjectContents { location: &project.location.clone() }
            }
        }
    )
}
