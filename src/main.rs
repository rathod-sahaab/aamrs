#![allow(non_snake_case)]
use dioxus::prelude::*;
use log::LevelFilter;
use resources::aamrs_config::AamrsConfig;
use resources::aamrs_state::AamrsState;
use ui::layouts::HomeLayout;
use ui::pages::Home;
use ui::pages::NewProject;
use crate::ui::components::modal::Modal;
use crate::ui::components::modal::ModalState;
pub mod files;
pub mod resources;
pub mod test_utils;
pub mod traits;
pub mod ui;
static CONFIG: GlobalSignal<AamrsConfig> = Signal::global(AamrsConfig::load_config);
static STATE: GlobalSignal<AamrsState> = Signal::global(AamrsState::load_state);
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/projects/new")]
    NewProject,
    #[layout(HomeLayout)]
    #[route("/")]
    Home {},
}
fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(
            r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string(),
        );
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}
#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(ModalState::default()));
    rsx! {
        Router::<Route> {}
        Modal {}
    }
}
