use crate::Route::*;
use dioxus::prelude::*;
use lucide_dioxus::{Dock, Megaphone, Settings};

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "hero min-h-[90vh] overflow-hidden",
            div {
                class: "hero-content text-center",
                div {
                    class: "max-w-md",
                    h1 {
                        class: "text-5xl font-bold",
                        "Oko Web"
                    }
                    p {
                        class: "py-6",
                        ""
                    }
                    div {
                        Link {
                            to: Bauchbinden {},
                            button {
                                class: "btn btn-outline btn-primary m-1",
                                Dock {}
                                "Bauchbinden"
                            }
                        }

                        Link {
                            to: Announcements {},
                             button {
                                class: "btn btn-outline btn-primary m-1",
                                Megaphone {}
                                "Announcements"
                            }
                        }
                        Link {
                            to: Settings {},
                            button {
                            class: "btn btn-outline btn-primary m-1",
                                Settings {}
                                "Settings"
                            }
                        }
                    }
                }
            }
        }
    }
}
