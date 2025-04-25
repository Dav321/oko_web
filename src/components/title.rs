use dioxus::prelude::*;
use lucide_dioxus::{Pencil, PencilOff};

#[component]
pub fn Title(title: String, edit: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "flex flex-row mt-4",
            h1 {
                class: "text-3xl font-bold mr-4",
                { title }
            }
            if edit.read().to_owned() {
                button {
                    class: "btn btn-square btn-base-content bg-base-300",
                    onclick: move |_| {
                        *edit.write() = false;
                    },
                    PencilOff {}
                }
            } else {
                button {
                    class: "btn btn-square btn-base-content bg-base-300",
                    onclick: move |_| {
                        *edit.write() = true;
                    },
                    Pencil {}
                }
            }
        }
    }
}
