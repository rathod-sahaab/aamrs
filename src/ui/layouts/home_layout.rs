use dioxus::prelude::*;

use crate::{ui::layouts::sidebar::Sidebar, Route};

#[component]
pub fn HomeLayout() -> Element {
    rsx!(
        div { class: "flex h-screen",
            div { class: "w-1/3 h-full flex flex-col", Sidebar {} }
            Outlet::<Route> {}
        }
    )
}
