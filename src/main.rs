mod components;
mod backend;

use components::{DogView, Favorites, NavBar};
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

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
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}