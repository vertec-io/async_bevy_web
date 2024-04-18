use std::{net::SocketAddr, sync::Arc};
// Websocket imports
use axum::{
    extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade}, http::{Error, StatusCode}, response::IntoResponse, Extension
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
use tokio::sync::mpsc;

use crate::web_server::WebServer;

pub async fn websocket_handler(
    
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    state: Extension<Arc<WebServer>>        
) -> impl IntoResponse {
    println!("Handling a new websocket connection!");
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    }else{
        String::from("Unknown browser")
    };
    println!("`{user_agent} at {addr} connected to {}.",state.server_name);

    // Finalize the upgrade process by returning upgrade callback.
    // We can customize the callback by sending additional info such as address
    ws.on_upgrade(move |socket| handle_socket(socket, addr, state))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(mut socket: WebSocket, who: SocketAddr, state: Extension<Arc<WebServer>>) {

    // Since each client gets individual websocket statemachine, we can pause handling
    // when necessary to wait from some external event. Here is where we can perform housekeeping 
    // tasks upon the initial connection, such as authentication, loading user settings from
    // the database, etc.
    if socket.send(Message::Text(format!("Hello from the server! Working to authenticate the user....")))
             .await
             .is_err()
        {
            println!("Client {who} abruptly disconnected");
            return;
        }
        // TODO: Authenticate the user here
        if let authenticated = authenticate_websocket_client(who, state.clone()).await {
            if socket.send(Message::Text(format!("Hello {who}! Welcome to {}",&state.server_name)))
                .await
                .is_err() {
                    println!("Client {who} abruptly disconnected");
                    return;
                }

        //TODO: Load user settings, etc....
        } else {
            if socket.send(Message::Text(format!("Hello {who}! Welcome to {}",&state.server_name)))
                .await
                .is_err() {
                    println!("Client {who} abruptly disconnected");
                    return;
                }   
        }

        // Split the websocket stream into a sender (sink) and receiver (stream)A
        // See example : https://gist.github.com/hexcowboy/8ebcf13a5d3b681aa6c684ad51dd6e0c
        //      NOTE: the link above is a solution for this general question: https://github.com/tokio-rs/axum/discussions/1159
        let (mut sink, mut stream) = socket.split();
        // Create an mpsc channel so we can send messages to the sink from multiple threads
        let (sender, mut receiver) = mpsc::channel::<Message>(32);

        // Spawn a task that forwards messages from the mpsc channel to the sink (client)
        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                if sink.send(message.into()).await.is_err(){
                    println!{"User disconnected"}
                    break;
                }
            }
        });

        //Subscribe to the Global broadcast channel
        let mut rx_global = state.tx.subscribe();

        // Whenever a message is sent to the Broadcast receiver, forward it to the mspc channel,
        // which will then forward it to the client
        let send_task_sender = sender.clone();
        let mut send_task = tokio::spawn(async move {
            while let Ok(msg) = rx_global.recv().await {
                if send_task_sender
                    .send(msg)
                    .await
                    .is_err(){
                        println!("User disconnected");
                        break;
                    }
            }
        });

        // Clone the global broadcast channel tx so we can send messages to it
        let tx_global = state.tx.clone();

        // Whenever a client sends a message to the websocket, forward it to the global broadcast
        let recv_task_sender = sender.clone();
        let mut recv_task = tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = stream.next().await {
                println!("{text}");
                let _ = tx_global.send(Message::Text(text));
                if recv_task_sender
                    .send(Message::Text(String::from("Your message has been sent")))
                    .await
                    .is_err() {
                        println!("User disconnected");
                        break;
                    }       
            }
        }); 

        // Clean up and close the conenctions if either task ends
        tokio::select! {
            _ = (&mut send_task) => recv_task.abort(),
            _ = (&mut recv_task) => send_task.abort(),
        };
}

async fn authenticate_websocket_client(who: SocketAddr, state: Extension<Arc<WebServer>>) -> bool {
        println!("Checking if user {who} is authenticated by {}", &state.server_name);
        // TODO: Authenticate the user here, replace the sleep with authentication tasks
        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
        return true
}