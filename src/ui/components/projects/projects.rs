use dioxus::prelude::*;

use crate::STATE;

#[component]
pub fn Projects() -> Element {
    // TODO: get from lazy static
    return rsx!(
        ul {
            for project in STATE.read().projects.iter() {
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
