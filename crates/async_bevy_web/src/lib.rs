use bevy::{app::{PluginGroupBuilder, 
    // ScheduleRunnerPlugin
}, prelude::*};
use bevy_tokio_tasks::TokioTasksPlugin;
use web_server::WebServerPlugin;
use bevy_framepace::FramepacePlugin;

use leptos::expect_context;
// use web_server::{WebServerPlugin};

use std::{sync::{Arc,Mutex}, time::Duration};

#[derive(Resource, Debug)]
pub struct FrameRate {
    value: f64,
}

impl FrameRate {
    fn new(value: f64) -> Self{
        Self{
            value
        }
    }
}

impl Default for FrameRate {
    fn default() -> FrameRate {
        FrameRate{
            value: 60.0
        }
    }
}

pub trait DataLayer{
    fn with_frame_rate(self, frame_rate: f64) -> Self;
    fn with_default_config(self)-> Self;
}

impl DataLayer for App{
    fn with_frame_rate(mut self, frame_rate: f64) -> Self {
         self
         .add_plugins(FramepacePlugin)
         .insert_resource(FrameRate::new(frame_rate))
         .add_systems(PostStartup, setup);

        //  .add_plugins(
        //      MinimalPlugins
        //             .set(ScheduleRunnerPlugin::run_loop(
        //             Duration::from_secs_f64(1./frame_rate)
        //             ))
        //     );
        self
    }
    
    fn with_default_config(mut self ) -> Self {
        self.add_plugins(MinimalPlugins)
            .add_plugins(AsyncBevyWebPlugins);
        self
    }
}

fn setup(mut settings: ResMut<bevy_framepace::FramepaceSettings>, frame_rate: Res<FrameRate>) {
    use bevy_framepace::Limiter;
    settings.limiter = Limiter::from_framerate(frame_rate.value);
}

pub struct AsyncBevyWebPlugins;

impl PluginGroup for AsyncBevyWebPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(TokioTasksPlugin::default())
            .add(WebServerPlugin)
            }
}