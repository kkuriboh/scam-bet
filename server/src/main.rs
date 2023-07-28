use axum::{routing::post, Router};
use fileserv::file_and_error_handler;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use web::App;

use routes::make_routes;

mod database;
mod fileserv;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

    let conf = get_configuration(None).await?;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    let web_app_router = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, |cx| view! { cx, <App/> })
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    let app_router = web_app_router.merge(make_routes().await?);

    let addr = "[::]:3000".parse()?;

    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app_router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
