use dioxus::prelude::*;

use crate::STATE;

#[component]
pub fn Projects() -> Element {
    // TODO: get from lazy static
    let state = &*STATE;

    return rsx!(
        ul {
            for project in &state.projects {
                li {
                    span {
                        class: "tooltip tooltip-right",
                        "data-tip": "{project.location}",
                        "{project.name}"
                    }
                }
            }
        }
    );
}
