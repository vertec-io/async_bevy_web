use std::time::Duration;

use bevy::prelude::*;
use async_bevy_web::DataLayer;
fn main(){
    App::new()
            .with_default_config()
            .with_frame_rate(6000000.0)
            .add_systems(Update,print_frame_time)
            .run();
}

fn print_frame_time(time: Res<Time>) {
    if time.delta() > Duration::from_micros(5000) {
        println!("Slow Frame: {:?}", time.delta());
    } else if time.delta() < Duration::from_nanos(4200) {
        println!("Fast Frame: {:?}", time.delta())
    }
    
}