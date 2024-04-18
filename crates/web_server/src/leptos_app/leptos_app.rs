use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use leptos::*;
use leptos_axum::{generate_route_list_with_exclusions_and_ssg_and_context, LeptosRoutes};
use leptos_router::build_static_routes_with_additional_context;

use crate::server::web_server::WebServer;
use crate::server::websocket::websocket_handler;

use std::sync::Arc;
use std::net::SocketAddr;
use axum::{
    routing::get,
    Router,
};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::AppState;

// Implement the LeptosView trait for any type that implements the bounds for Leptos and Bevy
pub trait LeptosView: IntoView + Send + Sync + 'static + Clone +Copy {}
impl<T> LeptosView for T where T: IntoView + Send + Sync + 'static + Clone +Copy {}
// pub trait BevySafe: Send + Sync + 'static + Clone + Copy {}

// impl<T> BevySafe for T where T: IntoView {}
// pub trait LeptosView: IntoView + BevySafe{}


#[derive(Resource, Debug, Clone, Copy)]
pub struct LeptosApp<F: LeptosView> {
    pub app_fn: F
}

impl<F> LeptosApp <F>
where
    F: LeptosView,
{
    pub fn new(app_fn:F) -> Self {
        Self {app_fn}
    }
}

pub struct LeptosAppPlugin<F>
where
    F: LeptosView
{
    leptos_app: LeptosApp<F>,
}

impl<F> LeptosAppPlugin <F>
where
    F: LeptosView
{
    pub fn new(app_fn: F) -> Self {
        let leptos_app = LeptosApp{app_fn};
        Self {
            leptos_app
        }
    }
}

impl <F> Plugin for LeptosAppPlugin<F>
where
    F: LeptosView
{
    fn build(&self, app: &mut App) {
        app.insert_resource(self.leptos_app.clone())
            .add_systems(PostStartup, start_leptos_app::<F>);
    }
}

fn start_leptos_app<F>(runtime: ResMut<TokioTasksRuntime>, server: Res<WebServer>, leptos_app:Res<LeptosApp<F>>)
where
    F: LeptosView +'static + Clone
{
    
    // Need to clone the server data to move it into the background task
    // Data from the server will communicate with Bevy over the channel
    // The socket connection will send serialized data from the client 
    // to communicate event types, data, status, etc.
    // We will use Bevy's event system will be used to trigger system
    // execution in the ECS. These systems will handle updating state, 
    // and returning data back to the client via the channel, which will get
    // relayed back via the websocket
    let server_clone = Arc::new(server.clone());
    let leptos_app_clone = Arc::new(leptos_app.clone());
    
    runtime.spawn_background_task(|mut _ctx| async move {
        let server_clone = (move || (server_clone))();    
        let server_clone2 = server_clone.clone();
        let server_clone3 = server_clone.clone();
        let socket_address = server_clone.address.clone();

        let leptos_app_clone = (move || leptos_app_clone)();
        let app_fn = leptos_app_clone.app_fn;
        let app_fn_clone = app_fn.clone();
        let app_fn_clone2 = app_fn.clone();

        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;

        let (routes, static_data_map) = generate_route_list_with_exclusions_and_ssg_and_context(
            move || {app_fn_clone}, 
            Some(vec!["/ws".into()]), 
            move || provide_context(server_clone.clone())
        );
        
        let leptos_options_clone = leptos_options.clone();
        let leptos_options_clone2 = leptos_options.clone();

        let routes_clone = routes.clone();
        let routes_clone2 = routes.clone();

        println!("Leptos Options: {:?}", &leptos_options);
        println!("Generated routes: {:?}", &routes_clone.clone());
        // Build static routes in a separate thread
        std::thread::spawn(move || {
            println!("Building static routes...");
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
                // .expect("Could not start a runtime to load static assets");

            rt.block_on(async {
                build_static_routes_with_additional_context(
                            &leptos_options_clone,
                            move || {app_fn_clone},
                            move || provide_context(server_clone2.clone()),
                            &routes_clone.clone(), 
                            &static_data_map
                        )
                        .await
                        .expect("Failed to build static routes")

            })
        });

        let app_state = AppState{
            server: server_clone3,
            leptos_options
        };

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
                                .route("/", get(root))
                                .route("/ws",get(websocket_handler))
                                .leptos_routes(&leptos_options_clone2, routes_clone2, move || app_fn_clone2)
                                .with_state(leptos_options_clone2)
                                // .fallback() <-- Need to add a fallback to my LeptosApp
                                .layer( //Logging setup
                                    TraceLayer::new_for_http()
                                        .make_span_with(DefaultMakeSpan::default().include_headers(true)),
                                );
        axum::serve(listener, axum_app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .expect("Server shut down unexpectedly");
    });
}

async fn root() -> &'static str {
    "Hello World! The application doesn't have anything on the root url"
}