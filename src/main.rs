#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
use resources::aamrs_config::AamrsConfig;
use resources::aamrs_state::AamrsState;

use ui::pages::home::home::Home;
use ui::pages::new_project::NewProject;

#[macro_use]
extern crate lazy_static;

pub mod files;
pub mod resources;
pub mod test_utils;
pub mod traits;
pub mod ui;

lazy_static! {
    pub static ref CONFIG: AamrsConfig = AamrsConfig::load_config();
    pub static ref STATE: AamrsState = AamrsState::load_state();
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/projects/new")]
    NewProject,
}

fn main() {
    println!("{:?}", *CONFIG);
    println!("{:?}", *STATE);

    // save_config(&CONFIG);
    // save_state(&STATE);
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
