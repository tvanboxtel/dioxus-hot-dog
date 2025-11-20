use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[derive(Clone)]
struct TitleState(String);

#[component]
fn App() -> Element {
    use_context_provider(|| TitleState("HotDog! 🌭".to_string()));
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}

    }
}

#[component]
fn Title() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
        div { id: "title",
            h1 { {title.0} }
        }
    }
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap();
        response.message
    });

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button { id: "skip", onclick: move |_| img_src.restart(), "Skip" }
            button { id: "save", onclick: move |_| img_src.restart(), "Save!" }
        }
    }
}
