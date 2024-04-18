use crate::files::folders::check_new_project_directory;
use crate::files::setup::create_project_in_directory;
use crate::resources::aamrs_state::AamrsProject;
use crate::ui::components::modal::ModalState;
use crate::STATE;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fi_icons::FiFolder;
use dioxus_free_icons::Icon;
use std::path::PathBuf;
fn updateState(project: AamrsProject) {
    (*STATE.write()).add_project(project);
    (*STATE.write()).save_state();
}
pub fn NewProject() -> Element {
    let mut loading = use_signal(|| false);
    let mut project_directory = use_signal(|| "".to_string());
    let mut project_directory_error = use_signal::<Option<String>>(|| None);
    let mut project_name = use_signal(|| "".to_string());

    let mut modal_state = consume_context::<Signal<ModalState>>();

    rsx! {
        div { class: "text-center w-full space-y-4",
            input {
                value: "{project_name}",
                placeholder: "Project name",
                class: "input input-bordered w-full",
                oninput: move |event| project_name.set(event.value())
            }
            button {
                class: "btn block w-full flex items-center justify-center space-x-2",
                disabled: loading(),
                onclick: move |_| {
                    spawn(async move {
                        loading.set(true);
                        if let Some(pd) = crate::ui::native::file_dialog::pick_project_directory()
                            .await
                        {
                            let pd_string = pd.to_str().unwrap().to_string();
                            project_directory.set(pd_string.clone());
                            if let Err(error) = check_new_project_directory(
                                &PathBuf::from(pd_string),
                            ) {
                                project_directory_error.set(Some(error.to_string()));
                            } else {
                                project_directory_error.set(None);
                            }
                        } else {
                            project_directory_error.set(Some("No folder selected".to_string()));
                        }
                        loading.set(false);
                    });
                },
                if loading() {
                    span { class: "loading loading-spinner" }
                } else {
                    Icon { icon: FiFolder }
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
            div {}
            button {
                class: "btn btn-primary w-full",
                disabled: project_directory_error().is_some(),
                onclick: move |_| {
                    if let Err(error) = create_project_in_directory(
                        PathBuf::from(project_directory()),
                    ) {
                        eprintln!("Error creating project: {}", error);
                    } else {
                        updateState(AamrsProject {
                            name: project_name(),
                            location: project_directory(),
                        });
                        modal_state.write().close();
                    }
                },
                "Create Project"
            }
        }
    }
}
