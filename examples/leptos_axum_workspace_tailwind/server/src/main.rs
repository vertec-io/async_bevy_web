use async_bevy_web::prelude::{ABWConfigPlugin, LeptosAppPlugin};
use bevy::prelude::*;

pub mod fileserv;
pub mod appserv;

use appserv::start_leptos_app;

fn main () {
    App::new()
        .add_systems(Startup, print_running)
        .add_plugins(ABWConfigPlugin::new(60.0))
        .add_plugins(LeptosAppPlugin::new(start_leptos_app()))
        .run();
}

fn print_running(){
    println!("Running!")
}