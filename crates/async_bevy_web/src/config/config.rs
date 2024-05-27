use bevy_app::{App, Plugin, PluginGroup, PluginGroupBuilder, PostStartup};
// use bevy_app::NoopPluginGroup as MinimalPlugins;
use bevy_ecs::prelude::{Resource, Res, ResMut};

// #[cfg(feature = "generator")]
use bevy_tokio_tasks::TokioTasksPlugin;

// #[cfg(feature = "generator")]
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

// #[cfg(feature = "generator")]
impl Plugin for ABWConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FramepacePlugin)
            .insert_resource(FrameRate::new(self.frame_rate))
            .add_systems(PostStartup, setup)
            .add_plugins(MinimalPlugins)
            .add_plugins(AsyncBevyWebPlugins);
    }
}

// #[cfg(feature = "generator")]
fn setup(mut settings: ResMut<bevy_framepace::FramepaceSettings>, frame_rate: Res<FrameRate>) {
    use bevy_framepace::Limiter;
    settings.limiter = Limiter::from_framerate(frame_rate.value);
}

// #[cfg(feature = "generator")]
pub struct AsyncBevyWebPlugins;
// #[cfg(feature = "generator")]
impl PluginGroup for AsyncBevyWebPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(TokioTasksPlugin::default())
            }
}

pub struct MinimalPlugins;

impl PluginGroup for MinimalPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(bevy_core::TaskPoolPlugin::default())
            .add(bevy_core::TypeRegistrationPlugin)
            .add(bevy_core::FrameCountPlugin)
            .add(bevy_time::TimePlugin)
            .add(bevy_app::ScheduleRunnerPlugin::default());
        // #[cfg(feature = "bevy_dev_tools")]
        // {
        //     group = group.add(bevy_dev_tools::DevToolsPlugin);
        // }
        group
    }
}