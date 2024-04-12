use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProjectContentsProps {
    location: String,
}

pub fn ProjectContents(project:ProjectContentsProps) -> Element {
    rsx!(
        span { "{project.location}" }
    )
}
