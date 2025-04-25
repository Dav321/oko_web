use components::*;
use dioxus::prelude::*;
use views::*;

mod backend;
mod components;
mod integrations;
mod views;

#[derive(Clone, Routable)]
#[rustfmt::skip]
enum Route {
    #[layout(Nav)]
    #[route("/")]
    Home {},
    #[route("/bb")]
    Bauchbinden {},
    #[route("/an")]
    Announcements {},
    #[route("/se")]
    Settings {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: CSS }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }

        Router::<Route> {}
    }
}
