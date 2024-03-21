use std::{env, net::{SocketAddr, ToSocketAddrs}};
use axum::{Router, routing::get};
use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use dotenv::dotenv;

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
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
    
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse::<u32>().unwrap_or(3000);
    let server_name = env::var("SERVER_NAME").unwrap_or_else(|_| "Axum Server".to_string());
    let new_addr = format!("{}:{}", host, port).to_socket_addrs().expect("Unable to parse socket address host and port").next().expect("Could not parse socket address and port");
    server.address = new_addr;
    server.port = port;
    server.server_name = server_name;
    server.socket = Some(format!("{}:{}", host, port));
    println!("Starting {} at host {} on port {}", &server.server_name, &server.address, &server.port);

    // Need to clone the server data to move it into the background task without affecting
    // the world state
    let server_clone = server.clone();
    
    runtime.spawn_background_task(|mut _ctx| async move {
        let server_clone = (move || (server_clone))();
        println!("{} successfully started on {}",&server_clone.server_name, &server_clone.address);
        
        let listener = tokio::net::TcpListener::bind(&server_clone.address).await.expect("Could not create TCP Listener");
        let axum_app = Router::new()
        .route("/", get(root));

        axum::serve(listener, axum_app)
            .await
            .expect("Server shut down unexpectedly");
    });
}

async fn root() -> &'static str {
    "Hello world!"
}