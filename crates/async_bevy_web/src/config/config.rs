use bevy::prelude::*;
use bevy::app::ScheduleRunnerPlugin;
use bevy_tokio_tasks::TokioTasksPlugin;
use std::time::Duration;

pub struct ABWConfigPlugin {
    frame_rate: f64,
}

impl Default for ABWConfigPlugin {
    fn default() -> Self {
        Self { frame_rate: 60.0 }
    }
}

impl ABWConfigPlugin {
    pub fn new(frame_rate: f64) -> Self {
        Self { frame_rate }
    }
}

impl Plugin for ABWConfigPlugin {
    fn build(&self, app: &mut App) {
        let frame_duration = Duration::from_secs_f64(1.0 / self.frame_rate);

        app.add_plugins(
                MinimalPlugins.set(
                    ScheduleRunnerPlugin::run_loop(frame_duration)
                )
            )
            .add_plugins(TokioTasksPlugin::default());
    }
}