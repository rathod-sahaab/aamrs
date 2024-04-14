use dioxus::prelude::*;
use dioxus_free_icons::icons::io_icons::IoClose;

#[derive(Clone, Default)]
pub struct ModalState{
    open: bool,
    heading: String,
    child: Option<Element>,
}

impl ModalState {
    pub fn close(&mut self) {
        println!("Close modal");
        self.open = false;
        self.heading.clear();
        self.child = None;
    }

    pub fn open(&mut self, heading: String, element: Element) {
        println!("Open modal");
        self.open = true;
        self.heading = heading;
        self.child = Some(element);
    }
}

pub fn Modal() -> Element {
    let mut modal_state = consume_context::<Signal<ModalState>>();

    rsx!(
        div { class: format!(
                "fixed bg-[#00000099] transition-all left-0 flex items-center justify-center h-screen w-screen {}",
                if !modal_state().open { "top-screen" } else { "top-0" },
            ),
            div { class: "bg-base-300 p-2",
                "{modal_state().heading}"
                button {
                    class: "btn btn-circle",
                    onclick: move |_| modal_state.write().close(),
                    dioxus_free_icons::Icon { icon: IoClose }
                }
                {modal_state().child.unwrap_or(rsx!()) }
            }
        }
    )
}
