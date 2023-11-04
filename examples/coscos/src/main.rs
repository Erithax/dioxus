use dioxus::prelude::*;

fn main() {
    // #[cfg(not(target_arch = "wasm32"))]
    // dioxus_desktop::launch(app);
    // #[cfg(target_arch = "wasm32")]
    dioxus_desktop::launch(app);
}

#[component]
fn app(cx: Scope) -> Element {
    render! {
        div { style: "color: blueviolet;",
            "Coscos in action."
            coscos { "background: coral;" }
        }
        div { "another one" }
    }
}
