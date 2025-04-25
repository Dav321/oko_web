use crate::Route;
use crate::Route::*;
use dioxus::prelude::*;
use lucide_dioxus::{Dock, Megaphone, Menu, Settings};

static LOGO: Asset = asset!("/assets/logo.png");

#[component]
pub fn Nav() -> Element {
    rsx!(
        div {
            class: "drawer",
            input {
                id: "nav-drawer",
                r#type: "checkbox",
                class: "drawer-toggle",
            }
            div {
                class: "drawer-content flex flex-col",
                div {
                    class: "navbar bg-base-300 w-full shadow-sm",
                    div {
                        class: "flex-none lg:hidden",
                        label {
                            for: "nav-drawer",
                            aria_label: "open sidebar",
                            class: "btn btn-square btn-ghost",
                            Menu {}
                        }
                    }
                    Link {
                        class: "flex flex-row flex-1",
                        to: Home {},
                        img {
                            class: "mx-2 w-10",
                            src: LOGO,
                        }
                        div {
                            class: "px-2 text-3xl",
                            "Oko Web"
                        }
                    }
                    div {
                        class: "hidden flex-none lg:block",
                        ul {
                            class: "menu menu-lg menu-horizontal",
                            li {
                                Link {
                                    to: Bauchbinden {},
                                    Dock {}
                                    "Bauchbinden"
                                }
                            }
                            li {
                                Link {
                                    to: Announcements {},
                                    Megaphone {}
                                    "Announcements"
                                }
                            }
                            li {
                                Link {
                                    to: Settings {},
                                    Settings {}
                                    "Settings"
                                }
                            }
                        }
                    }
                }
                Outlet::<Route> {}
            }
            div {
                class: "drawer-side",
                label {
                    for: "nav-drawer",
                    aria_label: "close sidebar",
                    class: "drawer-overlay"
                }
                ul {
                    class: "menu menu-xl bg-base-200 min-h-full w-80 p-4",
                    li {
                        Link {
                            to: Bauchbinden {},
                            Dock {}
                            "Bauchbinden"
                        }
                    }
                    li {
                        Link {
                            to: Announcements {},
                            Megaphone {}
                            "Announcements"
                        }
                    }
                    li {
                        Link {
                            to: Settings {},
                            Settings {}
                            "Settings"
                        }
                    }
                }
            }
        }
        footer {
            class: "fixed bottom-0 left-0",
            "©2025 David Schmidt"
        }
    )
}
