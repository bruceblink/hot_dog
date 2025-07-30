use dioxus::prelude::*;
use crate::backend::{list_dogs, remove_dog};

#[component]
pub fn Favorites() -> Element {
    let mut favorites = use_resource(list_dogs);

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (id , url) in favorites.suspend()?.cloned().unwrap() {
                    div { class: "favorite-dog", key: "{id}",
                        img { src: "{url}" }
                        button {
                            onclick: move |_| async move {
                                remove_dog(id).await.unwrap();
                                favorites.restart();
                            },
                            "❌"
                        }
                    }
                }
            }
        }
    }
}