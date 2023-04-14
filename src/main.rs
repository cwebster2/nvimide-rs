#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(App)
}

fn Sidebar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Sidebar"
        }
    })
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            Sidebar {}
            "Hello, world!"
        }
    })
}
