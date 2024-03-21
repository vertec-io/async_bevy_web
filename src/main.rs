use std::time::Duration;
use bevy::{app::ScheduleRunnerPlugin,prelude::*};

mod web_server;
use web_server::WebServerPlugin;

const FRAME_RATE:f64 = 60.0;


#[tokio::main]
async fn main() {

    App::new()
        .add_plugins(MinimalPlugins
            .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1./FRAME_RATE))))
        .add_plugins(WebServerPlugin)
        .run();

}

