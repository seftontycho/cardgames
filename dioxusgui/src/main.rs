use dioxus::{html::header, prelude::*};

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        header {
            title: "Card Games"
        }

        div {
            class: "title-container",
            h1 {
                class: "title",
                "Card Games"
            }
        }

        div {
            class: "button-container",
            button {
                class: "button1",
                "Button 1"
            }
            button {
                class: "button2",
                "Button 2"
            }
        }
    })
}
