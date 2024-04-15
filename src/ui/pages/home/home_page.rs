use dioxus::prelude::*;
pub fn Home() -> Element {
    rsx! {
        div {
            h1 { "Home component" }
        }
    }
}
