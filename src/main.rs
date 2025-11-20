use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
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
    let mut img_src = use_signal(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");
    let skip = move |evt| {};
    let save = move |evt| {
        img_src.set("https://images.dog.ceo/breeds/puggle/IMG_151824.jpg",);
    };
    rsx! {
        div { id: "dogview",
            img { src: "{img_src}" }
        }
        div { id: "buttons",
            button { id: "skip", onclick: skip, "Skip" }
            button { id: "save", onclick: save, "Save!" }
        }
    }
}

