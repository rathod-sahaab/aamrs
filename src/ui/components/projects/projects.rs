use dioxus::prelude::*;

use crate::files::state::load_state;

#[component]
pub fn Projects() -> Element {
    // TODO: get from lazy static
    let state = load_state();

    return rsx!(ul {
        for project in state.projects {
            li {
                span {
                    class: "tooltip tooltip-right",
                    "data-tip":"{project.location}",
                    "{project.name}"
                },
            }
        }
    });
}
