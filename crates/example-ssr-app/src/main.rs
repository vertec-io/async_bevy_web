use tokio::runtime::Runtime;
use axum::Router;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use example_ssr_app::app::*;
use example_ssr_app::fileserv::file_and_error_handler;
use std::net::SocketAddr;

#[cfg(feature = "ssr")]
fn main() {
    
    let (app, addr) = build_app();
    
    println!("Built the app to listen on on http://{}", &addr);

}

fn build_app() -> (Router, SocketAddr) {

    let rt = Runtime::new().unwrap(); // Creates a new Tokio runtime

    // Synchronously read the leptos configuration data from cargo.toml
    let conf = rt.block_on(async {
        get_configuration(None).await.unwrap()
    });

    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    (app, addr)
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
