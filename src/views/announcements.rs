use crate::backend::{add_announcement, clear_announcement, edit_announcement_name, edit_announcement_text, get_announcements, remove_announcement, show_announcement};
use dioxus::prelude::*;
use lucide_dioxus::{Minus, Pencil, PencilOff, Plus};

fn get_message(selected: i32, token: &str, list: &Vec<(i32, String, String)>) -> String {
    let mut res = "".to_string();
    for (id, _, text) in list {
        if *id == selected {
            res = text.replace("{}", token);
        }
    }
    res
}

#[component]
pub fn Announcements() -> Element {
    let mut data = use_server_future(get_announcements)?;
    let mut edit = use_signal(|| false);

    let mut selected = use_signal(|| 0);
    let mut token = use_signal(|| "".to_string());

    let mut add_name = use_signal(|| "".to_string());
    let mut add_text = use_signal(|| "".to_string());

    rsx! {
        div {
            class: "mx-4",
            class: "mx-4",
            div {
                class: "flex flex-row mt-4",
                h1 {
                    class: "text-3xl font-bold mr-4",
                    "Announcements"
                }
                if edit.read().to_owned() {
                    button {
                        class: "btn btn-square btn-base-content bg-base-300",
                        onclick: move |_| {
                            *edit.write() = false;
                            data.restart();
                        },
                        PencilOff {}
                    }
                } else {
                    button {
                        class: "btn btn-square btn-base-content bg-base-300",
                        onclick: move |_| {
                            *edit.write() = true;
                            data.restart();
                        },
                        Pencil {}
                    }
                }
            }
            if let Ok(list) = data.value().unwrap() {
                div {
                    class: "mt-4 flex w-full flex-col lg:flex-row",
                    div {
                        class: "overflow-x-auto card bg-base-300 rounded-box grid grow",
                        div {
                            class: "card-body items-center text-center",
                            h2 {
                                class: "card-title text-xl",
                                "Select"
                            }
                            table {
                                class: "table",
                                thead {
                                    tr {
                                        th {}
                                        th { "Name" }
                                        th { "Text" }
                                    }
                                }
                                tbody {
                                    if *edit.read() {
                                        tr {
                                            th {
                                                label {
                                                    button {
                                                        class: "btn btn-primary btn-square",
                                                        onclick: move |_| {
                                                            let name = (*add_name.read()).clone();
                                                            let text = (*add_text.read()).clone();

                                                            async move {
                                                                add_announcement(name, text).await.expect("Unable to add announcement");
                                                                data.restart();
                                                            }
                                                        },
                                                        Plus {}
                                                    }
                                                }
                                            }
                                            td {
                                                input {
                                                    r#type: "text",
                                                    class: "input input-primary",
                                                    oninput: move |event| {
                                                        *add_name.write() = event.data.value();
                                                    }
                                                }
                                            }
                                            td {
                                                input {
                                                    r#type: "text",
                                                    class: "input input-primary",
                                                    oninput: move |event| {
                                                        *add_text.write() = event.data.value();
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    for (id, name, text) in list.clone() {
                                        tr {
                                            if *edit.read() {
                                                th {
                                                    label {
                                                        button {
                                                            class: "btn btn-error btn-square",
                                                            onclick: move |_| async move {
                                                                remove_announcement(id).await.expect("Unable to remove announcement");
                                                                data.restart();
                                                            },
                                                            Minus {}
                                                        }
                                                    }
                                                }
                                                td {
                                                    input {
                                                        r#type: "text",
                                                        value: name,
                                                        class: "input input-ghost",
                                                        oninput: move |event| async move {
                                                            edit_announcement_name(event.value(), id).await.expect("Unable to edit name");
                                                        }
                                                    }
                                                }
                                                td {
                                                    input {
                                                        r#type: "text",
                                                        value: text,
                                                        class: "input input-ghost",
                                                        oninput: move |event| async move {
                                                            edit_announcement_text(event.value(), id).await.expect("Unable to edit name");
                                                        }
                                                    }
                                                }
                                            } else {
                                                th {
                                                    label {
                                                        input {
                                                            r#type: "radio",
                                                            name: "announcement",
                                                            class: "radio radio-primary",
                                                            checked: *selected.read() == id,
                                                            oninput: move |_| {
                                                                *selected.write() = id;
                                                            }
                                                        }
                                                    }
                                                }
                                                td { { name } }
                                                td { { text } }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "divider lg:divider-horizontal"
                    }
                    div {
                        class: "card bg-base-300 rounded-box grid",
                        div {
                            class: "card-body items-center text-center",
                            h2 {
                                class: "card-title text-xl",
                                "Input"
                            }
                            fieldset {
                                class: "fieldset w-full",
                                legend {
                                    class: "fieldset-legend",
                                    { "Value for \"{}\"".to_string() }
                                }
                                input {
                                    r#type: "text",
                                    class: "input w-full",
                                    oninput: move |event| {
                                        *token.write() = event.data.value();
                                    }
                                }
                            }
                            div {
                                class: "card card-border border-primary w-full",
                                div {
                                    class: "card-body items-center text-center p-0",
                                    h2 {
                                        class: "card-title m-2 text-base font-normal",
                                        { get_message(*selected.read(), &token.read(), &list) }
                                    }
                                }
                            }
                            div {
                                class: "flex flex-row w-full",
                                button {
                                    class: "btn btn-primary grow mr-2",
                                    onclick: move |_| {
                                        let msg = get_message(*selected.read(), &token.read(), &list);
                                        async move {
                                            let _ = clear_announcement().await;
                                            show_announcement(msg).await.expect("Unable to add announcement");
                                        }
                                    },
                                    "Show"
                                }
                                button {
                                    class: "btn btn-error grow ml-2",
                                    onclick: move |_| async move {
                                        clear_announcement().await.expect("Unable to clear announcement");
                                    },
                                    "Clear"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
