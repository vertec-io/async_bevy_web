
use bevy_ecs::prelude::{Res, ResMut};
use bevy_tokio_tasks::{TaskContext, TokioTasksRuntime};

use leptos::*;
use leptos_axum::{generate_route_list_with_exclusions, LeptosRoutes};

use std::sync::Arc;
use std::net::SocketAddr;
use axum::{
    routing::get,
    Router,
};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
};
use tower::ServiceExt;
use tower_http::services::ServeDir;
use axum::response::Response as AxumResponse;
use axum_macros::FromRef;
use crate::{LeptosApp, LeptosView};
use web_server::{websocket_handler, WebServer};

#[derive(FromRef, Debug, Clone)]
pub struct AppState{
    pub leptos_options: LeptosOptions,
    pub world_context: TaskContext,
    pub server: Arc<WebServer>
}

pub fn start_leptos_app<F>(
    runtime: ResMut<TokioTasksRuntime>, 
    server: Res<WebServer>, 
    leptos_app:Res<LeptosApp<F>>,
)
where
    F: LeptosView +'static + Clone
{
    let server_clone = Arc::new(server.clone());
    let leptos_app_clone = Arc::new(leptos_app.clone());
    
    runtime.spawn_background_task(|ctx| async move {

        let leptos_app_clone = (move || leptos_app_clone)();
        let app_fn = leptos_app_clone.app_fn;
        let app_fn_clone = app_fn.clone();
        let app_fn_clone2 = app_fn.clone();

        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let leptos_options_clone = leptos_options.clone();
        let socket_address = &leptos_options.site_addr;

        let app_state = AppState {
            leptos_options: leptos_options_clone,
            world_context: ctx,
            server: server_clone,
        };

        let routes = generate_route_list_with_exclusions(move || {app_fn_clone}, Some(vec!["/ws".into()]));

        println!("Leptos Options: {:?}", &leptos_options);
        println!("Generated routes: {:?}", &routes);

        tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "web_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

        let listener = tokio::net::TcpListener::bind(&socket_address).await.expect("Could not create TCP Listener");

        println!("Starting server on a new thread, listening at {}", &socket_address);
        let axum_app: Router = Router::new()
                                .route("/ws",get(websocket_handler))
                                .leptos_routes_with_context(
                                    &app_state,
                                    routes,
                                    {
                                        let app_state = app_state.clone();
                                        move || provide_context(app_state.clone())
                                    },
                                    move || app_fn_clone2)
                                .fallback(file_and_error_handler)
                                .with_state(app_state)
                                .layer( //Logging setup
                                    TraceLayer::new_for_http()
                                        .make_span_with(DefaultMakeSpan::default().include_headers(true)),
                                );
        axum::serve(listener, axum_app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .expect("Server shut down unexpectedly");
    });
}


pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    _req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();
    if res.status() == StatusCode::NOT_FOUND {
        // try with `.html`
        let uri_html = format!("{}.html", uri).parse().unwrap();
        get_static_file(uri_html, &root)
            .await
            .unwrap()
            .into_response()
    } else {
        res.into_response()
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}