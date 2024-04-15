use dioxus::prelude::*;
use dioxus_free_icons::icons::io_icons::IoClose;

#[derive(Clone)]
pub struct ModalState{
    open: bool,
    heading: String,
    child: fn()->Element,
}

impl Default for ModalState {
    fn default() -> Self {
        Self { open: Default::default(), heading: Default::default(), child: || None}
    }
}

impl ModalState {
    pub fn close(&mut self) {
        self.open = false;
        self.heading.clear();
        self.child = || rsx!(  );
    }

    pub fn open(&mut self, heading: String, element: fn()->Element) {
        self.open = true;
        self.heading = heading;
        self.child = element;
    }
}

pub fn Modal() -> Element {
    let mut modal_state = consume_context::<Signal<ModalState>>();
    let Comp = modal_state().child;

    rsx!(
        div { class: format!(
                "fixed bg-[#0009] backdrop-blur-sm left-0 flex items-center justify-center h-screen w-screen {}",
                if !modal_state().open { "top-screen" } else { "top-0" },
            ), onclick: move |_| modal_state.write().close(),
            div {
                class: "bg-base-100 p-4 relative rounded-md",
                onclick: move |event| {
                    event.stop_propagation();
                },
                h1 { class: "text-2xl font-bold pb-2", "{modal_state().heading}" }
                button {
                    class: "btn btn-circle btn-error absolute right-0 top-0 translate-x-1/2 -translate-y-1/2",
                    onclick: move |event| {
                        event.stop_propagation();
                        modal_state.write().close()
                    },
                    dioxus_free_icons::Icon { icon: IoClose }
                }
                div { class: "p-4 pb-0 min-w-screen-md", Comp {} }
                // Double Comp {} required I don't know why
                div { class: "hidden", Comp {} }
            }
        }
    )
}
