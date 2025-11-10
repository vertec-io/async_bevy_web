# async-bevy-web

A Bevy plugin that provides a minimal configuration setup for running Bevy applications in headless mode with integrated Tokio runtime and Leptos support for building real-time web applications.

## Features

- **Headless Bevy**: Runs Bevy with `MinimalPlugins` (no rendering/windowing)
- **Configurable Frame Rate**: Set your desired frame rate for the Bevy ECS loop
- **Time Mode Control**: Choose between variable or fixed timestep for deterministic behavior
- **Tokio Integration**: Built-in Tokio runtime via `bevy-tokio-tasks`
- **Leptos Support**: Integration with Leptos for server-side rendering via `bevy-leptos`

## How To

### Basic Setup (Variable Timestep)

Add the plugin to your Bevy app with a desired frame rate:

```rust
use async_bevy_web::prelude::*;

fn main() {
    App::new()
        .add_plugins(ABWConfigPlugin::new(60.0)) // 60 FPS, variable timestep
        .run();
}
```

### Fixed Timestep (Deterministic)

For applications requiring deterministic timing (robotics, simulations):

```rust
use async_bevy_web::prelude::*;

fn main() {
    App::new()
        .add_plugins(ABWConfigPlugin::fixed(20.0)) // 20 Hz fixed timestep
        .add_systems(FixedUpdate, my_deterministic_system) // Runs at exactly 20 Hz
        .run();
}
```

### Advanced Configuration

You can explicitly specify the time mode:

```rust
use async_bevy_web::prelude::*;

fn main() {
    App::new()
        // Variable timestep - good for web servers, UI
        .add_plugins(ABWConfigPlugin::with_mode(60.0, TimeMode::Variable))
        // OR
        // Fixed timestep - good for robotics, physics, deterministic simulations
        .add_plugins(ABWConfigPlugin::with_mode(20.0, TimeMode::Fixed))
        .run();
}
```

### Default Configuration

You can also use the default configuration (60 FPS, variable timestep):

```rust
use async_bevy_web::prelude::*;

fn main() {
    App::new()
        .add_plugins(ABWConfigPlugin::default())
        .run();
}
```

### Time Mode Comparison

| Mode | Use Case | Systems Schedule | Timing Behavior |
|------|----------|------------------|-----------------|
| **Variable** | Web servers, UI, non-critical timing | `Update` | Frame rate is a target, actual delta varies with system load |
| **Fixed** | Robotics, physics, deterministic simulations | `FixedUpdate` | Systems run at exact intervals, multiple updates per frame if needed |

**Recommendation for Robotics**: Use `TimeMode::Fixed` with a lower frame rate (10-20 Hz) and put your control logic in the `FixedUpdate` schedule. Use Tokio background tasks for async I/O with hardware.

## What's Included

The `ABWConfigPlugin` automatically sets up:

1. **Bevy MinimalPlugins** - Core Bevy functionality without rendering
   - TaskPoolPlugin
   - FrameCountPlugin
   - TimePlugin
   - ScheduleRunnerPlugin (with configured frame rate)

2. **TokioTasksPlugin** - Tokio runtime integration for async tasks

3. **Leptos Support** - Via the `bevy-leptos` crate for web server integration

## Version Compatibility

| async-bevy-web version | bevy version | bevy-tokio-tasks version | bevy-leptos version | Rust version |
|---|---|---|---|---|
| 0.3.0 | 0.17.0 | 0.17.0 | 0.1.0 | nightly (1.88.0+) |
| 0.3.0 | 0.16.0 | 0.16.0 | 0.1.0 | stable |

## Requirements

- **Rust nightly (1.88.0 or later)** - Required for Bevy 0.17's use of Rust Edition 2024 features
- The project includes a `rust-toolchain.toml` file that will automatically use the correct toolchain

## Examples

See the [examples directory](../../examples) for complete working examples:

- `leptos_axum_workspace_tailwind` - Full-stack Leptos + Axum + Tailwind CSS example
- `ssr_leptix_tailwind` - Server-side rendering with Tailwind
- `example_app` - Basic example application

## License

This project is licensed under the same terms as the parent repository.

