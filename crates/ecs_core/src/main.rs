// use std::time::Duration;
use bevy::{app::ScheduleRunnerPlugin,prelude::*};
use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};
use web_server::{WebServerPlugin, WebServer};

use std::time::Duration;

use axum::extract::ws::Message;

const FRAME_RATE:f64 = 60.0;
fn main() {
    App::new()
        .add_plugins(MinimalPlugins
            .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1./FRAME_RATE)))
        )
        .add_plugins(TokioTasksPlugin::default())
        .add_plugins(WebServerPlugin)
        .add_systems(Startup, demo)
        .add_systems(Update, (countdown, time_done))
        .init_resource::<AppTime>()
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
        let msg = "Five more seconds have elapsed on the main thread".to_string();
        println!("{msg}");
        let tx_sender = server.tx.clone();
        
        runtime.spawn_background_task(|mut _ctx| async move {
            let _ = tx_sender.send(Message::Text(msg));
        });
    }
}
