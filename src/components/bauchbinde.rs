use crate::backend::{edit_name, remove_name, show_bauchbinde};
use dioxus::prelude::*;
use lucide_dioxus::Trash2;
use std::collections::BTreeMap;

#[derive(PartialEq, Props, Clone)]
pub struct BauchbindeProps {
    id: i32,
    name: String,
    section: String,
    edit: Signal<bool>,
    data: Resource<Result<BTreeMap<i32, (String, Vec<(i32, String)>)>, ServerFnError>>,
}

#[component]
pub fn Bauchbinde(mut props: BauchbindeProps) -> Element {
    rsx! {
        div {
            class: "card card-border bg-base-300 border-primary",
            div {
                class: "card-body items-center text-center p-0",
                h2 {
                    class: "card-title m-2 mb-0",
                    if *props.edit.read() {
                        input {
                            r#type: "text",
                            class: "input, input-ghost",
                            value: props.name.clone(),
                            oninput: move |event| async move {
                                edit_name(event.value(), props.id).await.expect("Unable to edit name");
                            }
                        }
                        button {
                        class: "btn btn-square btn-base-content bg-base-300",
                        onclick: move |_| async move {
                            remove_name(props.id).await.expect("Unable to remove section");
                            props.data.restart();
                        },
                        Trash2 {}
                    }
                    } else {
                        { props.name.clone() }
                    }
                }
                div {
                    class: "card-actions flex-nowrap w-full p-1",
                    button {
                        class: "btn btn-outline btn-primary w-full rounded-xl",
                        onclick: move |_| {
                            let name = props.name.clone();
                            let section = props.section.clone();
                            async move {
                                show_bauchbinde(name, section).await.expect("Could not Show Bauchbinde!");
                            }
                        },
                        "Show"
                    }
                }
            }
        }
    }
}
