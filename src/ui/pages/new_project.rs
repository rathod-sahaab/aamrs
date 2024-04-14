use std::path::PathBuf;

use dioxus::prelude::*;
use dioxus_free_icons::icons::io_icons::IoClose;
use dioxus_free_icons::icons::io_icons::IoFolder;
use dioxus_free_icons::Icon;

use crate::files::folders::checkNewProjectDirectory;
use crate::files::setup::createProjectInDirectory;
use crate::resources::aamrs_state::AamrsProject;
use crate::Route;
use crate::STATE;

#[component]
pub fn NewProject() -> Element {
    let mut loading = use_signal(|| false);

    let nav = navigator();

    let mut project_directory = use_signal(|| "".to_string());
    let mut project_directory_error = use_signal::<Option<String>>(|| None);
    let mut project_name = use_signal(|| "".to_string());
    rsx! {
        section { class: "flex items-center justify-center",
            div { class: "text-center max-w-md space-y-4 p-8 m-4",
                h1 { class: "text-2xl font-bold pb-2", "Create a new project" }
                input {
                    value: "{project_name}",
                    placeholder: "Project name",
                    class: "input input-bordered w-full",

                    oninput: move |event| project_name.set(event.value())
                }
                button {
                    class: "btn block w-full flex items-center justify-center space-x-2",
                    onclick: move |_| {
                        spawn(async move {
                            loading.set(true);
                            if let Some(pd) = crate::ui::native::file_dialog::pick_project_directory()
                                .await
                            {
                                let pd_string = pd.to_str().unwrap().to_string();
                                project_directory.set(pd_string.clone());
                                if let Err(error) = checkNewProjectDirectory(&PathBuf::from(pd_string)) {
                                    project_directory_error.set(Some(error.to_string()));
                                } else {
                                    project_directory_error.set(None);
                                }
                            }
                            loading.set(false);
                        });
                    },
                    if loading() {
                        span { class: "loading loading-spinner" }
                    } else {
                        Icon { icon: IoFolder }
                    }
                    if project_directory.to_string().is_empty() {
                        "Select Directory"
                    } else {
                        "{project_directory}"
                    }
                }
                if project_directory_error().is_some() {
                    span { class: "text-error", "{project_directory_error.unwrap()}" }
                }
                button {
                    class: "btn btn-primary",
                    disabled: project_directory_error().is_some(),
                    onclick: move |_| {
                        if let Err(error) = createProjectInDirectory(
                            PathBuf::from(project_directory()),
                        ) {
                            eprintln!("Error creating project: {}", error);
                        } else {
                            (*STATE.write())
                                .add_project(AamrsProject {
                                    name: project_name(),
                                    location: project_directory(),
                                });
                            (*STATE.write()).save_state();
                            nav.replace(Route::Home {});
                        }
                    },
                    "Create Project"
                }
            }
            button {
                class: "absolute btn btn-circle top-8 right-8",
                onclick: move |_| {
                    nav.replace(Route::Home {});
                },
                Icon { icon: IoClose }
            }
        }
    }
}
