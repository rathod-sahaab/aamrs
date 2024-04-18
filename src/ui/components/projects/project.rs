use crate::ui::components::projects::dir_contents::DirContents;
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::{
        fi_icons::{FiChevronDown, FiChevronRight, FiFolderPlus, FiTrash},
        io_icons::IoEllipsisVertical,
    },
    Icon,
};
#[derive(Props, Clone, PartialEq)]
pub struct ProjectProps {
    name: String,
    location: String,
}
pub fn Project(project: ProjectProps) -> Element {
    let mut is_expanded = use_signal(|| false);
    let mut context_menu_open = use_signal(|| false);

    let project_location = use_signal(|| project.location);

    rsx!(
        li {
            class: "flex flex-col pl-4 pr-2 group relative",
            onclick: move |_| is_expanded.set(!is_expanded()),
            div {
                class: "tooltip tooltip-right w-full text-left flex justify-between",
                "data-tip": "{project_location}",
                span {
                    if is_expanded() {
                        dioxus_free_icons::Icon { class: "inline", icon: FiChevronDown }
                    } else {
                        dioxus_free_icons::Icon { class: "inline", icon: FiChevronRight }
                    }
                    span { "{project.name}" }
                }
                button {
                    class: "btn btn-sm btn-ghost btn-circle opacity-0 group-hover:opacity-100",
                    onclick: move |event| {
                        event.stop_propagation();
                        context_menu_open.set(!context_menu_open());
                    },
                    dioxus_free_icons::Icon { class: "inline", icon: IoEllipsisVertical }
                }
            }
            div { class: format!("overflow-hidden {}", if is_expanded() { "" } else { "max-h-0" }),
                DirContents { directory: project_location()}
            }
            ul {
                // context menu
                class: format!(
                    "absolute top-0 left-full menu bg-base-300 w-56 rounded-box {}",
                    if context_menu_open() { "" } else { "hidden" },
                ),
                li {
                    button {
                        onclick: move |event| {
                            event.stop_propagation();
                            context_menu_open.set(false);
                            println!("location: {}", project_location());
                        },
                        Icon { icon: FiFolderPlus }
                        "Create folder"
                    }
                    button {
                        onclick: move |event| {
                            event.stop_propagation();
                            context_menu_open.set(false);
                            println!("location: {}", project_location());
                        },
                        Icon { icon: FiTrash }
                        "Remove Project"
                    }
                }
            }
        }
    )
}
