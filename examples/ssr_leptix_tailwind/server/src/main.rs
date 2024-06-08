use async_bevy_web::prelude::{ABWConfigPlugin, LeptosAppPlugin};
use bevy::{prelude::*, tasks::{TaskPool, TaskPoolBuilder}};
use bevy_eventwork::{ConnectionId, EventworkRuntime, Network, NetworkData, NetworkEvent};
use bevy_eventwork_mod_websockets::{WebSocketProvider, NetworkSettings};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use shared::messages::*;

pub mod fileserv;
pub mod appserv;
pub mod messages;

use appserv::start_leptos_app;

fn main () {
    let mut app = App::new();

    // Base ABW plugins
    app.add_systems(Startup, print_running)
       .add_plugins(ABWConfigPlugin::new(60.0))
       .add_plugins(LeptosAppPlugin::new(start_leptos_app()));

    // Networking plugins    
    app.add_plugins(bevy_eventwork::EventworkPlugin::<WebSocketProvider,bevy::tasks::TaskPool,>::default())
       .insert_resource(EventworkRuntime(TaskPoolBuilder::new().num_threads(2).build(),))
       .insert_resource(NetworkSettings::default());
    
    // Register the messages where they are defined
    messages::server_register_network_messages(&mut app);

    // Networking systems
    app.add_systems(Startup, setup_networking)
        .add_systems(Update, (handle_connection_events, handle_messages));
    
    // Start the app
    app.run();
}

fn print_running(){
    println!("Running!")
}

// On the server side, you need to setup networking. You do not need to do so at startup, and can start listening
// at any time.
fn setup_networking(
    mut net: ResMut<Network<WebSocketProvider>>,
    settings: Res<NetworkSettings>,
    task_pool: Res<EventworkRuntime<TaskPool>>,
) {
    let ip_address = "127.0.0.1".parse().expect("Could not parse ip address");

    println!("Address of the server: {}:8080", ip_address);

    let _socket_address = SocketAddr::new(ip_address, 8080);

    match net.listen(
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081),
        &task_pool.0,
        &settings,
    ) {
        Ok(_) => (),
        Err(err) => {
            error!("Could not start listening: {}", err);
            panic!();
        }
    }

    println!("Started listening for new connections!");
}


#[derive(Component)]
struct User(ConnectionId);

fn handle_connection_events(
    mut commands: Commands,
    net: Res<Network<WebSocketProvider>>,
    mut network_events: EventReader<NetworkEvent>,
){
    for event in network_events.read(){
        if let NetworkEvent::Connected(conn_id) = event {
            commands.spawn((User(*conn_id),));

        // Broadcasting sends the message to all connected users! (Including the just connected one in this case)
        net.broadcast(NewChatMessage {
            name: String::from("SERVER"),
            message: format!("New user connected: {}", conn_id),
        });
        info!("New user connected: {}",conn_id);
        }
    }
}

fn handle_messages(
    mut new_messages: EventReader<NetworkData<UserChatMessage>>,
    net: Res<Network<WebSocketProvider>>,
){
    for message in new_messages.read(){
        let user = message.source();
        info!("Received message from user: {}", message.message);

        net.broadcast(NewChatMessage{
            name: format!("{}",user),
            message: message.message.clone(),
        });
    }
}