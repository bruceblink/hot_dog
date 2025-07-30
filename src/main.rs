use dioxus::{logger::tracing::info, prelude::*};

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone)]
struct TitleState(String);

#[component]
fn App() -> Element {
    // Provide that type as a Context
    use_context_provider(|| TitleState("HotDog! 🌭".to_string()));
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
        div { id: "title",
             h1 { "{title.0}" }
        }
    }
}


#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            button { onclick: move |_| img_src.restart(), id: "save", "save!" }
        }
    }
}
