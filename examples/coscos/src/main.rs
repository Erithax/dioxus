use dioxus::prelude::*;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    dioxus_desktop::launch(app);
    #[cfg(target_arch = "wasm32")]
    dioxus_web::launch(app2);
}

#[component]
fn app2(cx: Scope) -> Element {
    render! { div { "minimal" } }
}

#[component]
fn app(cx: Scope) -> Element {
    let counter = use_state(cx, || 2);

    render! {
        div { style: "color: blueviolet;",
            "Coscos in action."
            coscos { "background: coral;" }
        }
        div { style: "display: flex; flex-direction: row; ",
            div {
                style: "border: 1px solid gray; padding: 20px; border-radius: 4px;",
                onclick: move |_| { counter.set(counter.get() + 1) },
                "UP"
            }
            div {
                style: "border: 1px solid gray; padding: 20px; border-radius: 4px;",
                onclick: move |_| { counter.set(if *counter.get() > 0 { counter.get() - 1 } else { 0 }) },
                "DOWN"
            }
        }
        if true {
            rsx!{div{}}
        } else {
            rsx!{div{}}
        }
        div {
            for i in 0..4 {
                div {}
            }
        }

        for i in 0..*counter.get() {
            Compy {}
        }
    }
}

#[component]
fn Compy(cx: Scope) -> Element {
    render! {
        div {
            "COMPY"
            coscos {
                "
                color: cyan;
                border: 1px solid black;
                border-radius: 5px;
                "
            }
        }
    }
}
