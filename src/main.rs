#![allow(non_snake_case)]

use dioxus::prelude::*;
use files::{config::load_config, state::load_state};
use log::LevelFilter;

use crate::files::{config::save_config, state::save_state};

use ui::pages::new_project::NewProject;

pub mod files;
pub mod resources;
pub mod test_utils;
pub mod traits;
pub mod ui;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/projects/new")]
    NewProject,
}

fn main() {
    let config = load_config();
    let state = load_state();

    println!("{:?}", config);
    println!("{:?}", state);

    save_config(&config);
    save_state(&state);
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        Link {
            to: Route::NewProject {},
            class: "btn btn-primary",
            "Create a new project"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { class: "btn btn-primary", onclick: move |_| count += 1, "Up high!" }
            button { class: "btn", onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
