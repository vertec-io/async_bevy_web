use bevy::prelude::*;
use bevy::app::ScheduleRunnerPlugin;
use bevy_tokio_tasks::TokioTasksPlugin;
use std::time::Duration;

/// Time control mode for the Bevy application
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeMode {
    /// Variable timestep - frame rate is a target, actual delta time varies
    /// Good for: UI, web servers, non-critical timing
    Variable,
    /// Fixed timestep - systems run at exact intervals regardless of frame timing
    /// Good for: Deterministic simulations, physics, robotics control loops
    Fixed,
}

/// Configuration for the async Bevy web application
pub struct ABWConfigPlugin {
    frame_rate: f64,
    time_mode: TimeMode,
}

impl Default for ABWConfigPlugin {
    fn default() -> Self {
        Self {
            frame_rate: 60.0,
            time_mode: TimeMode::Variable,
        }
    }
}

impl ABWConfigPlugin {
    /// Create a new config with variable timestep (default behavior)
    ///
    /// # Arguments
    /// * `frame_rate` - Target frames per second (e.g., 60.0 for 60 FPS)
    pub fn new(frame_rate: f64) -> Self {
        Self {
            frame_rate,
            time_mode: TimeMode::Variable,
        }
    }

    /// Create a new config with specified time mode
    ///
    /// # Arguments
    /// * `frame_rate` - Target frames per second (e.g., 60.0 for 60 FPS)
    /// * `time_mode` - TimeMode::Variable or TimeMode::Fixed
    ///
    /// # Example
    /// ```
    /// use async_bevy_web::prelude::*;
    ///
    /// // For deterministic robotics control at 20 Hz
    /// let config = ABWConfigPlugin::with_mode(20.0, TimeMode::Fixed);
    /// ```
    pub fn with_mode(frame_rate: f64, time_mode: TimeMode) -> Self {
        Self {
            frame_rate,
            time_mode,
        }
    }

    /// Create a new config with fixed timestep
    ///
    /// # Arguments
    /// * `frame_rate` - Fixed update rate in Hz (e.g., 20.0 for 50ms updates)
    pub fn fixed(frame_rate: f64) -> Self {
        Self::with_mode(frame_rate, TimeMode::Fixed)
    }

    /// Create a new config with variable timestep
    ///
    /// # Arguments
    /// * `frame_rate` - Target frame rate in Hz (e.g., 60.0 for ~16.67ms frames)
    pub fn variable(frame_rate: f64) -> Self {
        Self::with_mode(frame_rate, TimeMode::Variable)
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

        // Configure fixed timestep if requested
        if self.time_mode == TimeMode::Fixed {
            app.insert_resource(Time::<Fixed>::from_hz(self.frame_rate));
        }
    }
}