# async-bevy-web

A Bevy plugin that provides a minimal configuration setup for running Bevy applications in headless mode with integrated Tokio runtime and Leptos support for building real-time web applications.

## Features

- **Headless Bevy**: Runs Bevy with `MinimalPlugins` (no rendering/windowing)
- **Configurable Frame Rate**: Set your desired frame rate for the Bevy ECS loop
- **Tokio Integration**: Built-in Tokio runtime via `bevy-tokio-tasks`
- **Leptos Support**: Integration with Leptos for server-side rendering via `bevy-leptos`

## How To

### Basic Setup

Add the plugin to your Bevy app with a desired frame rate:

```rust
use async_bevy_web::prelude::*;

fn main() {
    App::new()
        .add_plugins(ABWConfigPlugin::new(60.0)) // 60 FPS
        .run();
}
```

### Default Configuration

You can also use the default configuration (60 FPS):

```rust
use async_bevy_web::prelude::*;

fn main() {
    App::new()
        .add_plugins(ABWConfigPlugin::default())
        .run();
}
```

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

| async-bevy-web version | bevy version | bevy-tokio-tasks version | bevy-leptos version |
|---|---|---|---|
| 0.3.0 | 0.16.0 | 0.16.0 | 0.1.0 |

## Examples

See the [examples directory](../../examples) for complete working examples:

- `leptos_axum_workspace_tailwind` - Full-stack Leptos + Axum + Tailwind CSS example
- `ssr_leptix_tailwind` - Server-side rendering with Tailwind
- `example_app` - Basic example application

## License

This project is licensed under the same terms as the parent repository.

