use crate::backend::{edit_section, remove_section};
use dioxus::html::completions::CompleteWithBraces::data;
use dioxus::prelude::*;
use lucide_dioxus::Trash2;
use std::collections::BTreeMap;

#[derive(PartialEq, Props, Clone)]
pub struct SectionProps {
    id: i32,
    name: String,
    edit: Signal<bool>,
    data: Resource<Result<BTreeMap<i32, (String, Vec<(i32, String)>)>, ServerFnError>>,
    children: Element,
}

#[component]
pub fn Section(mut props: SectionProps) -> Element {
    rsx!(
        div {
            h2 {
                class: "text-2xl mt-4",
                if *props.edit.read() {
                    input {
                        r#type: "text",
                        class: "input, input-ghost",
                        value: props.name,
                        oninput: move |event| async move {
                            edit_section(event.value(), props.id).await.expect("Unable to edit section");
                        }
                    }
                    button {
                        class: "btn btn-square btn-base-content bg-base-300",
                        onclick: move |_| async move {
                            remove_section(props.id).await.expect("Unable to remove section");
                            props.data.restart();
                        },
                        Trash2 {}
                    }
                } else {
                    { props.name }
                }
            }
            { props.children }
        }
    )
}
