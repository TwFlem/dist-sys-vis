use leptos::{html::Canvas, leptos_dom::console_log, *};
use leptos_meta::*;
use leptos_router::*;
use std::f64;
use wasm_bindgen::prelude::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Canvas />
    }
}

#[component]
fn Canvas(cx: Scope) -> impl IntoView {
    let canvas_ref = create_node_ref::<Canvas>(cx);
    canvas_ref.on_load(cx, move |cr| {
        let context = cr
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        context.begin_path();

        for i in 0..1000000 {
            let anchor = (i) as f64 * 0.001;
            context
                .arc(
                    75.0 * anchor,
                    75.0 * anchor,
                    50.0 * anchor,
                    0.0,
                    f64::consts::PI * 2.0,
                )
                .unwrap();
        }
        context.stroke();
    });
    view! { cx,
        <canvas _ref=canvas_ref />
    }
}
