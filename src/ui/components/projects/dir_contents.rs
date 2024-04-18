use std::path::PathBuf;

use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fi_icons::{FiFile, FiFolder},
    Icon,
};

use crate::files::folder_contents::{get_folder_contents, Entry};
#[derive(Props, Clone, PartialEq)]
pub struct ProjectContentsProps {
    directory: String,
}
pub fn DirContents(props: ProjectContentsProps) -> Element {
    let entries = get_folder_contents(&PathBuf::from(props.directory));

    if entries.is_err() {
        return rsx!( span {
            class: "text-errror",
            "Error reading directory",
        });
    }

    let entries_rendered = entries
        .unwrap()
        .into_iter()
        .map(|entry| match entry {
            Entry::Dir(dir) => rsx!(
                Icon {
                    class: "inline",
                    icon: FiFolder
                }
                span { "{dir.name}"}
            ),
            Entry::File(file) => rsx!(
                Icon {
                    class: "inline",
                    icon: FiFile
                }
                span { "{file.name}"}
            ),
        })
        .map(|component| {
            rsx!(
                div {
                    // row
                    class: "space-x-2",
                    {component}
                }
            )
        });

    rsx! {
        div{
            class: "p-4 bg-base-100 space-y-3",
            {entries_rendered}
        }
    }
}
