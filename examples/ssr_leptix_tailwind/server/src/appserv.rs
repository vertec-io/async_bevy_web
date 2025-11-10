use app::*;
use axum::Router;
use fileserv::file_and_error_handler;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use async_bevy_web::prelude::leptos_app;

use crate::fileserv;

#[leptos_app]
pub async fn start_leptos_app() {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(MyApp);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, MyApp)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}