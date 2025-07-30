mod components;
mod backend;

use components::{DogView, Favorites, NavBar};
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,
}

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}