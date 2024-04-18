
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_tokio_tasks::TokioTasksPlugin;

use web_server::WebServerPlugin;
use bevy_framepace::FramepacePlugin;


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

pub trait Config{
    fn with_frame_rate(self, frame_rate: f64) -> Self;
    fn with_default_config(self)-> Self;
    // fn with_leptos_app(self ) -> Self;
    fn start(self) -> Self;
}

impl Config for App{
    fn with_frame_rate(mut self, frame_rate: f64) -> Self {
         self
         .add_plugins(FramepacePlugin)
         .insert_resource(FrameRate::new(frame_rate))
         .add_systems(PostStartup, setup);

        self
    }
    
    fn with_default_config(mut self ) -> Self {
        self.add_plugins(MinimalPlugins)
            .add_plugins(AsyncBevyWebPlugins);
        // .add_plugins(WebServerPlugin::default(new));
        self
    }

    // fn with_leptos_app(mut self) -> Self {
    //     self.insert_resource(LeptosApp {});
    //     self
    // }

    fn start(mut self) -> Self {
        self.add_plugins(WebServerPlugin);
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