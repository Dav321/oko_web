use dioxus::prelude::*;

#[component]
pub fn Setting(name: String, id: String, value: String) -> Element {
    rsx! {
        label {
            class: "fieldset-label text-sm",
            { name.as_str() }
        }
        input {
            class: "input text-base w-full",
            name: id,
            value: value,
            placeholder: name.as_str(),
        }
    }
}
