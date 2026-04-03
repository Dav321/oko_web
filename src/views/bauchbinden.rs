use crate::backend::{
    add_name, add_section, get_bauchbinden, get_vmix_titles, set_bauchbinde_text,
};
use crate::components::*;
use dioxus::prelude::*;
use lucide_dioxus::{Image, ImageOff, Pencil, PencilOff, Plus};

#[component]
pub fn Bauchbinden() -> Element {
    let mut data = use_server_future(get_bauchbinden)?;
    let mut vmix_xml = use_server_future(get_vmix_titles)?;

    let mut edit = use_signal(|| false);
    let mut multi = use_signal(|| false);
    let mut selected: Signal<Option<(String, (String, Vec<String>))>> = use_signal(|| None);

    rsx! {
        div {
            class: "mx-4",
            class: "mx-4",
            div {
                class: "flex flex-row mt-4",
                h1 {
                    class: "text-3xl font-bold mr-4",
                    "Bauchbinden"
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
                if multi.read().to_owned() {
                    button {
                        class: "btn btn-square btn-base-content bg-base-300",
                        onclick: move |_| {
                            *multi.write() = false;
                            vmix_xml.restart();
                        },
                        ImageOff {}
                    }
                } else {
                    button {
                        class: "btn btn-square btn-base-content bg-base-300",
                        onclick: move |_| {
                            *multi.write() = true;
                            vmix_xml.restart();
                        },
                        Image {}
                    }
                }
            }
            if *multi.read() {
                if let Ok(map) = vmix_xml.value().unwrap() {
                    div {
                        class: "dropdown",
                        div {
                            tabindex: 0,
                            class: "btn m-1",
                            role: "button",
                            if let Some((_, (name, _))) = (*selected.read()).clone() {
                                { name }
                            } else {
                                "Select GT LowerThird"
                            }
                        }
                        ul {
                            tabindex: 0,
                            class: "dropdown-content menu bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm",
                            for item in map {
                                li {
                                    a {
                                        onclick: move |_| {
                                            let i = item.clone();
                                            *selected.write() = Some(i);
                                        },
                                        { item.1.0.clone() },
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some((_, (_, texts))) = (*selected.read()).clone() {
                    form {
                        onsubmit: move |event: FormEvent| {
                            event.prevent_default();
                            let (_, (_, t2)) = (*selected.read()).clone().unwrap();
                            async move {
                                for t in t2 {
                                    let FormValue::Text(value) = event.data.get_first(&t).unwrap() else {
                                        panic!("Unable to set BB Text: value not of type String");
                                    };
                                    set_bauchbinde_text(t.clone(), value).await.expect("Unable to set BB Text");
                                }
                            }
                        },
                        table {
                            class: "table",
                                thead {
                                tr {
                                    th { "Field" }
                                    th { "Value" }
                                }
                            }
                            tbody {
                                for t in texts {
                                    tr {
                                        th {
                                            label {
                                                { t.clone() }
                                            }
                                        }
                                        td {
                                            input {
                                                r#type: "text",
                                                class: "input",
                                                name: t,
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        button {
                            class: "btn btn-primary mt-4",
                            r#type: "submit",
                            "Send"
                        }
                    }
                }
            } else {
                if let Ok(section) = data.value().unwrap() {
                    for (section_id, (section_name, list)) in section {
                        Section {
                            id: section_id,
                            name: section_name.to_owned(),
                            edit: edit,
                            data: data,
                            div {
                                class: "grid grid-cols-5 gap-4",
                                for (person_id, person_name) in list {
                                    Bauchbinde {
                                        id: person_id,
                                        name: person_name,
                                        section: section_name.clone(),
                                        edit: edit,
                                        data: data,
                                    }
                                }
                                if edit.read().to_owned() {
                                    button {
                                        class: "card card-border bg-base-300 border-primary",
                                        onclick: move |_|  {
                                            async move {
                                                add_name("Unbennant".to_string(), section_id).await.expect("Could not create Section!");
                                                data.restart();
                                            }
                                        },
                                        figure {
                                            class: "h-full",
                                            Plus {
                                                size: 64
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if edit.read().to_owned() {
                        button {
                            class: "btn btn-outline btn-base-content bg-base-300 mt-4",
                            onclick: move |_| async move {
                                add_section("Unbennant".to_string()).await.expect("Could not create Section!");
                                data.restart();
                            },
                            Plus {}
                        }
                    }
                }
            }
        }
    }
}
