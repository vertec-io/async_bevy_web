use async_bevy_web::prelude::{ABWConfigPlugin, LeptosAppPlugin};
use bevy::prelude::*;

pub mod appserv;

use appserv::start_leptos_app;

fn main () {
    App::new()
        .add_systems(Startup, print_running)
        // Use lower frame rate for headless web server (reduces CPU usage and OS scheduler interaction)
        .add_plugins(ABWConfigPlugin::fixed(60.0))
        .add_plugins(LeptosAppPlugin::new(start_leptos_app()))
        .run();
}

fn print_running(){
    println!("Running!")
}