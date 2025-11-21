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

#[post("/api/save_dog")]
async fn save_dog(image: String) -> Result<()> {
    use std::io::Write;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap();

    file.write_fmt(format_args!("{image}\n"));

    Ok(())
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
            button {
                id: "save",
                onclick: move |_| async move {
                    let current = img_src.cloned().unwrap();
                    img_src.restart();
                    _ = save_dog(current).await;
                },
                "Save!"
            }
        }
    }
}
