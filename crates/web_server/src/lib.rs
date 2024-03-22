use std::{env, net::{SocketAddr, ToSocketAddrs}};
use axum::{
    routing::get,
    Router, 
};

use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use dotenv::dotenv;

// Websocket imports
use axum::{
    extract::{ws::{Message, WebSocket},
        WebSocketUpgrade},
    response::IntoResponse, 
    Extension
};

use axum_extra::TypedHeader;
use std::borrow::Cow;
use std::ops::ControlFlow;
use std::path::PathBuf;
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

//Allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;

//Allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};




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
struct WebServer {
    address: SocketAddr,
    port: u32,
    server_name: String,
    socket: Option<String>
}

impl Default for WebServer {
    fn default() -> WebServer {
        WebServer {
            address: SocketAddr::from(([127,0,0,1], 3000)),
            port: 3000,
            server_name: String::from("Axum Server"),
            socket: None
        }
    }
}

fn start_server(runtime: ResMut<TokioTasksRuntime>, mut server: ResMut<WebServer>) {
    //READ environment variables for the host and port and update the server address
    dotenv().ok();    
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse::<u32>().unwrap_or(3000);
    let server_name = env::var("SERVER_NAME").unwrap_or_else(|_| "Axum Server".to_string());
    let new_addr = format!("{}:{}", host, port).to_socket_addrs().expect("Unable to parse socket address host and port").next().expect("Could not parse socket address and port");
    println!("Environment Variables for server:\n NAME: {}:\n HOST: {} \n PORT: {}", &server_name, &host, &port);
    
    server.address = new_addr;
    server.port = port;
    server.server_name = server_name;
    server.socket = Some(format!("{}:{}", host, port));
    
    // Need to clone the server data to move it into the background task
    let server_clone = server.clone();
    
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
        let axum_app = Router::new()
        .route("/", get(root))
        .route("/ws",get(handle_websocket))
        .layer( //Logging setup
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

        println!("Starting {} on a new thread, listening on address {}",&server_clone.server_name, &server_clone.address);
        axum::serve(listener, axum_app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .expect("Server shut down unexpectedly");
    });
}

async fn root() -> &'static str {
    "Hello world!"
}

async fn handle_websocket(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    }else{
        String::from("Unknown browser")
    };
    println!("`{user_agent} at {addr} connected.");

    // Finalize the upgrade process by returning upgrade callback.
    // We can customize the callback by sending additional infor such as address
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                    Message::Text(t) => {
                        println!(">>> {who} sent str: {t:?}");
                    }
                    _ => println!("Unsupported message"),
                };
        }
}
}