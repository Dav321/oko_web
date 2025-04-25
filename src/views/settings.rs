use crate::backend::{get_announcements, get_propresenter, get_vmix, get_vmix_titles, set_propresenter, set_vmix};
use crate::components::{Setting, Title};
use crate::integrations::{ProPresenter, Vmix};
use dioxus::prelude::*;
use tracing::debug;

#[component]
pub fn Settings() -> Element {
    let propresenter = use_server_future(get_propresenter)?;
    let vmix = use_server_future(get_vmix)?;
    let vmix_xml = use_server_future(get_vmix_titles)?;

    rsx!(
        div {
            class: "mx-4",
            div {
                class: "flex flex-row mt-4",
                h1 {
                    class: "text-3xl font-bold mr-4",
                    "Settings"
                }
            }
            div {
                class: "flex w-full flex-col lg:flex-row",
                if let Ok(value) = propresenter.value().unwrap() {
                    form {
                        class: "grow",
                        onsubmit: move |e: Event<FormData>| {
                            let values = e.values();
                            let pp = ProPresenter::new(
                                values.get("url").unwrap().as_value(),
                                values.get("message_name").unwrap().as_value(),
                                values.get("theme_name").unwrap().as_value(),
                                values.get("theme_index").unwrap().as_value().parse().unwrap(),
                                values.get("theme_uuid").unwrap().as_value()
                            );
                            async move {
                                set_propresenter(pp).await.expect("Unable to set propresenter Settings");
                            }
                        },
                        fieldset {
                            class: "fieldset bg-base-200 border border-base-300 p-4 rounded-box",
                            legend {
                                class: "fieldset-legend text-xl",
                                "ProPresenter"
                            }
                            Setting {
                                name: "IP + Port",
                                id: "url",
                                value: value.get_pro_presenter_url(),
                            }
                            Setting {
                                name: "Message Name",
                                id: "message_name",
                                value: value.get_message_name(),
                            }
                            Setting {
                                name: "Theme Name",
                                id: "theme_name",
                                value: value.get_theme_name(),
                            }
                            Setting {
                                name: "Theme Index",
                                id: "theme_index",
                                value: value.get_theme_index(),
                            }
                            Setting {
                                name: "Theme UUID",
                                id: "theme_uuid",
                                value: value.get_theme_uuid(),
                            }
                            button {
                                class: "btn btn-primary mt-4",
                                r#type: "submit",
                                "Save"
                            }
                        }
                    }
                }
                div {
                    class: "divider lg:divider-horizontal invisible"
                }
                if let Ok((value, (name_field, title_field))) = vmix.value().unwrap() {
                    form {
                        class: "grow",
                        onsubmit: move |e: Event<FormData>| {
                            let values = e.values();
                            let vm = Vmix::new(
                                values.get("url").unwrap().as_value(),
                                values.get("overlay_index").unwrap().as_value().parse().unwrap(),
                                values.get("object_uuid").unwrap().as_value()
                            ).unwrap();
                            let name_field = values.get("name_field").unwrap().as_value();
                            let title_field = values.get("title_field").unwrap().as_value();
                            async move {
                                set_vmix(vm, name_field, title_field).await.expect("Unable to set vmix Settings");
                            }
                        },
                        fieldset {
                            class: "fieldset bg-base-200 border border-base-300 p-4 rounded-box",
                            legend {
                                class: "fieldset-legend text-xl",
                                "Vmix"
                            }
                            Setting {
                                name: "IP + Port",
                                id: "url",
                                value: value.get_vmix_url(),
                            }
                            Setting {
                                name: "Overlay Index (1-8)",
                                id: "overlay_index",
                                value: value.get_overlay_index() as i32,
                            }
                            Setting {
                                name: "Object UUID",
                                id: "object_uuid",
                                value: value.get_object_uuid(),
                            }
                            Setting {
                                name: "Name Field",
                                id: "name_field",
                                value: name_field,
                            }
                            Setting {
                                name: "Title Field",
                                id: "title_field",
                                value: title_field,
                            }
                            button {
                                class: "btn btn-primary mt-4",
                                r#type: "submit",
                                "Save"
                            }
                            if let Ok(map) = vmix_xml.value().unwrap() {
                                div {
                                    class: "card bg-base-300 mt-4 w-full",
                                    div {
                                        class: "card-body",
                                        code {
                                        for (uuid, (name, texts)) in map {
                                            { name.to_owned() + ":" }
                                            br {}
                                            { "- UUID: ".to_owned() + &*uuid }
                                            br {}
                                            { "- Fields: ".to_owned() + texts.join(", ").as_str() }
                                            br {}
                                        }
                                    }
                                    }

                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
