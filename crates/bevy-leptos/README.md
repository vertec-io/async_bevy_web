# bevy-leptos

A Bevy plugin that provides integration between Bevy ECS and Leptos web framework, enabling you to build real-time, interactive web applications with Bevy as the data layer.

## Features

- **Leptos Integration**: Seamlessly integrate Leptos server-side rendering with Bevy
- **Async Support**: Built on top of `bevy-tokio-tasks` for async runtime support
- **Macro Support**: Convenient macros via `abw_macros` for common patterns

## How To

### Using the `leptos_app` Macro

The `leptos_app` macro simplifies setting up a Leptos application with Bevy:

```rust
use async_bevy_web::prelude::*;

#[leptos_app]
pub async fn start_leptos_app() {
    // Your Leptos + Axum setup here
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(MyApp);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, MyApp)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
```

The `#[leptos_app]` macro automatically:
1. Sets up the Bevy app with `ABWConfigPlugin`
2. Spawns your async function as a Tokio task
3. Runs the Bevy ECS loop alongside your web server

## Version Compatibility

| bevy-leptos version | bevy version | bevy-tokio-tasks version | leptos version |
|---|---|---|---|
| 0.1.0 | 0.16.0 | 0.16.0 | 0.6.x |

## Examples

See the [examples directory](../../examples) for complete working examples:

- `leptos_axum_workspace_tailwind` - Full-stack Leptos + Axum + Tailwind CSS example
- `ssr_leptix_tailwind` - Server-side rendering with Tailwind

## Dependencies

This crate depends on:
- `bevy` - The Bevy game engine
- `bevy-tokio-tasks` - Tokio runtime integration
- `abw_macros` - Procedural macros for convenience

## License

This project is licensed under the same terms as the parent repository.

