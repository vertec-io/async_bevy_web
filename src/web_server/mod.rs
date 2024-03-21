use std::{env, net::{SocketAddr, ToSocketAddrs}};

use axum::{Router, routing::get};
use bevy::prelude::*;




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


fn start_server(mut server: ResMut<WebServer>) {
    //READ environment variables for the host and port and update the server address
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse::<u32>().unwrap_or(3000);
    let server_name = env::var("SERVER_NAME").unwrap_or_else(|_| "Axum Server".to_string());
    let new_addr = format!("{}:{}", host, port).to_socket_addrs().expect("Unable to parse socket address host and port").next().expect("Could not parse socket address and port");
    server.address = new_addr;
    server.port = port;
    server.server_name = server_name;
    server.socket = Some(format!("{}:{}", host, port));
    println!("Starting the server at host {} on port {}", &server.address, &server.port);

    let server_clone = server.clone();

    std::thread::spawn(move || {
        //initialize tracing
        tracing_subscriber::fmt::init();
        
        let runtime = tokio::runtime::Runtime::new().expect("Could not start the server async runtime");
        runtime.block_on(async move {
            // build our application with a route
        let axum_app = Router::new()
            // `GET /` goes to `root`
            .route("/", get(root));
            
        let listener = tokio::net::TcpListener::bind(&server_clone.address).await.expect("Could not create TCP Listener");
        println!("Axum server successfully started on {}", &server_clone.address);
        axum::serve(listener, axum_app)
            .await
            .expect("Server shut down unexpectedly");
    });
    });
}

async fn root() -> &'static str {
    "Hello world!"
}