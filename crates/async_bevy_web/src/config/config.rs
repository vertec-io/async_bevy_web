
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

pub struct ABWConfigPlugin {
    frame_rate: f64,
}

impl Default for ABWConfigPlugin {
    fn default() -> Self {
        Self {frame_rate: 60.0}
    }
}

impl ABWConfigPlugin{
    pub fn new(frame_rate: f64) -> Self{
        Self { frame_rate }
    }
}

impl Plugin for ABWConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FramepacePlugin)
            .insert_resource(FrameRate::new(self.frame_rate))
            .add_systems(PostStartup, setup)
            .add_plugins(MinimalPlugins)
            .add_plugins(AsyncBevyWebPlugins);
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