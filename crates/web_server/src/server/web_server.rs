
use std::net::SocketAddr;
use axum::extract::ws::Message;
use bevy::prelude::*;
use tokio::sync::broadcast;

#[derive(Component)]
pub struct WebServerPlugin;

// #[cfg(feature = "ssr")]
impl Plugin for WebServerPlugin{
    fn build(&self, app: &mut App) {
        app
        .init_resource::<WebServer>();
    }
}

#[derive(Resource, Clone)]
pub struct WebServer {
    pub address: SocketAddr,
    pub port: u32,
    pub server_name: String,
    pub socket_address: Option<String>,
    pub tx: broadcast::Sender<Message>, // Channel used to send messages to all connected clients    
}

impl Default for WebServer {
    fn default() -> WebServer {
        let (tx, _) = broadcast::channel(32);
        WebServer {
            address: SocketAddr::from(([127,0,0,1], 3000)),
            port: 3000,
            server_name: String::from("Axum Server"),
            socket_address: None,
            tx,
        }
    }
}
