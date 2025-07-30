use dioxus::{logger::tracing::info, prelude::*};

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let skip = move |evt| {info!("Clicked skip ! Event: {evt:?}")};

    let mut img_src = use_signal(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");

    let save = move |evt| {
        info!("Clicked save ! Event: {evt:?}");
        img_src.set("https://images.dog.ceo/breeds/leonberg/n02111129_974.jpg");
    };
    rsx! {
        div { id: "dogview",
            img { src: "{img_src}" }
        }
        div { id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            button { onclick: save, id: "save", "save!" }
        }
    }
}
