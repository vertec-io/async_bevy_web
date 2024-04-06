use bevy::prelude::*;
use async_bevy_web::DataLayer;
use bevy_tokio_tasks::TokioTasksRuntime;
use web_server::WebServer;
use axum::extract::ws::Message;
use std::time::Duration;

fn main(){
    App::new()
            .with_default_config()
            .with_frame_rate(60.0)
            .init_resource::<AppTime>()
            .add_systems(Startup, demo)
            .add_systems(Update, (countdown, time_done))
            .run();
}

fn demo(runtime: ResMut<TokioTasksRuntime>, server: ResMut<WebServer>) {
    let tx_sender = server.tx.clone();
    runtime.spawn_background_task(|mut _ctx| async move {
        let mut index = 0;
        loop {
            
            let msg = format!("Seconds elapsed on a background thread {:?}", index);
            println!("{msg}");
            
            let _ = tx_sender.send(Message::Text(msg));
            
            index += 1;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
}

#[derive(Resource)]
struct AppTime (pub Timer);

impl AppTime {
    pub fn new() -> Self {
        Self(Timer::from_seconds(5.0, TimerMode::Repeating))
    }
}

impl Default for AppTime {
    fn default() -> Self {
        Self::new()
    }
}

fn countdown(
    time: Res<Time>,
    mut app_time: ResMut<AppTime>
){
    app_time.0.tick(time.delta());
}

fn time_done(app_time: Res<AppTime>, runtime: ResMut<TokioTasksRuntime>, server: ResMut<WebServer>){
    if app_time.0.finished() {
        // Print a message on in the bevy runtime locally
        let msg = "Five more seconds have elapsed on the main thread".to_string();
        println!("{msg}");

        // Send a message to the webserver clients
        let _ = server.tx.send(Message::Text(msg.clone()));
        
        //Alternatively send the server message in a background thread:
        let tx_sender = server.tx.clone();
        runtime.spawn_background_task(|mut _ctx| async move {
            let _ = tx_sender.send(Message::Text(msg));
        });
    }
}
