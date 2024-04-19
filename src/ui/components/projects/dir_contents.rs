use std::path::PathBuf;

use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::{
        fi_icons::{FiFile, FiFolder, FiFolderPlus, FiTrash},
        io_icons::IoEllipsisVertical,
    },
    Icon,
};

use crate::files::folder_contents::{get_folder_contents, Entry};

#[component]
fn ContextMenu(path: Signal<String>, context_menu_open: Signal<bool>) -> Element {
    rsx!(
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
                        println!("location: {}", path());
                    },
                    Icon { icon: FiFolderPlus }
                    "Create folder"
                }
                button {
                    onclick: move |event| {
                        event.stop_propagation();
                        context_menu_open.set(false);
                        println!("location: {}", path());
                    },
                    Icon { icon: FiTrash }
                    "Remove Project"
                }
            }
        }
    )
}

#[component]
fn DirRoot(name: String, path: Signal<String>) -> Element {
    let mut context_menu_open = use_signal(|| false);
    rsx! {
        div {
            class: "flex justify-between w-full group relative",
            div {
                class: "space-x-2",
                Icon {
                    class: "inline",
                    icon: FiFolder
                }
                span { "{name}" },
            }
            button {
                class: "btn btn-sm btn-ghost btn-circle opacity-0 group-hover:opacity-100",
                onclick: move |event| {
                    event.stop_propagation();
                    context_menu_open.set(!context_menu_open());
                },
                dioxus_free_icons::Icon { class: "inline", icon: IoEllipsisVertical }
            }
            ContextMenu {path, context_menu_open}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ProjectContentsProps {
    name: String,
    directory: PathBuf,
    #[props(default = true)]
    render_root: bool,
    is_expanded: Option<Signal<bool>>,
}

pub fn Directory(props: ProjectContentsProps) -> Element {
    let mut is_expanded = props.is_expanded.unwrap_or(use_signal(|| false));

    let path = use_signal(|| props.directory.to_str().unwrap_or("<None>").to_string());

    rsx! {
        div{
            class: "space-y-2",
            onclick: move |event| {
                event.stop_propagation();
                is_expanded.set(!is_expanded());
            },
            if props.render_root {
                DirRoot {name: props.name.clone(), path}
            }
            if is_expanded() {
                Entries {directory: props.directory}
            }
        }
    }
}

#[component]
fn Entries(directory: PathBuf) -> Element {
    let entries = get_folder_contents(&directory);

    if entries.is_err() {
        return rsx!( span {
            class: "text-error",
            "Error reading directory",
        });
    }

    let entries_rendered = entries.unwrap().into_iter().map(|entry| match entry {
        Entry::Dir(dir) => rsx!(Directory {
            directory: dir.path,
            name: dir.name.clone()
        }),
        Entry::File(file) => rsx!(
            div {
                // row
                class: "space-x-2",
                Icon {
                    class: "inline",
                    icon: FiFile
                }
                span { "{file.name}"}
            }
        ),
    });

    rsx! {
        div {
            class: "pl-2 space-y-2",
            {entries_rendered}
        }
    }
}
