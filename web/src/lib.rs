use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/scam-bet.css"/>
        <Title text="Welcome to Leptos"/>
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <Routes>
                <Route path="/" view=|cx| view! { cx, <HomePage/> }/>
                <Route path="/roulette" view=|cx| view! { cx, <Roulette/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let on_click = move |_| {
        let nav = use_navigate(cx);
        request_animation_frame(move || nav("/roulette", Default::default()).unwrap());
    };

    view! { cx,
        <main>
            <h1>"Welcome to Leptos!"</h1>
            <button on:click=on_click>"roulette"</button>
        </main>
    }
}

#[component]
fn Roulette(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <script src="games/roulette.js" />
    }
}
