use std::{env, net::{SocketAddr, ToSocketAddrs}, sync::Arc};
use axum::{
    routing::get,
    Router,
    extract::ws::Message
};

use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use dotenv::dotenv;

use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tokio::sync::{broadcast, mpsc};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod websocket;
use websocket::websocket_handler;

#[derive(Component)]
pub struct WebServerPlugin;

impl Plugin for WebServerPlugin{
    fn build(&self, app: &mut App) {
        app
        .init_resource::<WebServer>()
        .add_systems(Startup, start_server);
    }
}


#[derive(Resource, Clone)]
pub struct WebServer {
    pub address: SocketAddr,
    pub port: u32,
    pub server_name: String,
    pub socket_address: Option<String>,
    pub tx: broadcast::Sender<Message> // Channel used to send messages to all connected clients
    
}

impl Default for WebServer {
    fn default() -> WebServer {
        let (tx, _) = broadcast::channel(32);
        WebServer {
            address: SocketAddr::from(([127,0,0,1], 3000)),
            port: 3000,
            server_name: String::from("Axum Server"),
            socket_address: None,
            tx
        }
    }
}

fn start_server(runtime: ResMut<TokioTasksRuntime>, mut server: ResMut<WebServer>) {
    //READ environment variables for the host and port and update the server address
    dotenv().ok();    
    // let host = env::var("HOST").unwrap();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse::<u32>().unwrap_or(3000);
    let server_name = env::var("SERVER_NAME").unwrap_or_else(|_| "Axum Server".to_string());
    let new_addr = format!("{}:{}", host, port).to_socket_addrs().expect("Unable to parse socket address host and port").next().expect("Could not parse socket address and port");

    println!("Environment Variables for server:\n NAME: {}:\n HOST: {} \n PORT: {}", &server_name, &host, &port);
    
    server.address = new_addr;
    server.port = port;
    server.server_name = server_name;
    server.socket_address = Some(format!("{}:{}/ws", host, port));
    
    println!("{:?}", usize::MAX/2);
    // Need to clone the server data to move it into the background task
    let server_clone = Arc::new(server.clone());
    
    runtime.spawn_background_task(|mut _ctx| async move {

        // Tracing
        tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "web_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

        let server_clone = (move || (server_clone))();    
        let listener = tokio::net::TcpListener::bind(&server_clone.address).await.expect("Could not create TCP Listener");

        println!("Starting {} on a new thread, listening on address {}",&server_clone.server_name, &server_clone.address);
        let axum_app = Router::new()
        .route("/", get(root))
        .route("/ws",get(websocket_handler))
        .with_state(server_clone)
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
    "Hello World!"
}