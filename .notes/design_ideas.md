# DESIGN IDEAS

This project has a lot of different components, and it's difficult to figure out how to build it without a few notes to keep track of ideas.

Here's a rough layout of the architectural components that are needed:

## Entity Component System (ECS)
This system handles all data processing and business logic in the application. It's designed to behave as an in-memory database with a data-driven architecture to ensure high performance and minimal data scattering. It's an alternative to Object Oriented Programming (OOP) and fits closer to the Rust programming language semantics

## Plugins
All functionality that is integrated into the underlying ECS is built using a plugin architecture. The backend event loop, the front end web server, websocket communication, IO, database queries, etc. are all plugins

### Core Plugins
There are several core plugins that make up the framework and enable building large complex multi-user, multi-threaded, asyncronous, web-enabled applications

#### Bevy MinimalPlugins
We're using the Bevy Game Engine crate to provide the fundamental ECS and event loop of the architecture. Bevy is a high performance, multi-threaded and parallel execution framework that provides one of the best ECS and is a fully Plugin-based architecture. The event loop can be executed by building an application with resources, systems, and all associated plugins, then calling the .run() method on the app. Bevy uses the Builder pattern to construct the application, which provides excellent ergonomics for developers.

MinimalPlugins is a group of plugins in Bevy that allow for running the system as a headless application - i.e. no render window. This is perfect for our use case because we won't be rendering a UI within the bevy backend, instead we'll be serving a Web Application that provides the user interface and websocket connections for clients. More on that later

#### Bevy-Tokio-Tasks
This fundamental plugin enables async I/O inside of the event loop so that we can pull external data from sensors, devices, machines, webserver/websocket communication, etc. without blocking the main Bevy event loop. Some of these operations may take multiple frames to complete, and we don't want the ECS waiting for data to come back for it to continue execution of its systems and events. Async execution is a pivotal core component of the framework. This plugin creates an Async Runtime using Tokio and provides it as a resource to the Bevy ECS. It includes a mutable world context so that functions and methods can be executed in the main bevy thread, as well as an API to spawn background tasks on the runtime.

#### WebServer
This Fundamental plugin creates an Axum webserver that runs in a background task on the async runtime provided by the Bevy-Tokio-Tasks plugin. It starts a server using configuration parameters (Host, Port, etc.) and configures a websocket that allows for realtime connection to the ECS backend. The applications built on this framework can define their endpoints and register them as events in the Bevy ECS so that when those endpoints are requested by clients, the appropriate methods run in the main world. In this way, web applications can simply define Bevy Entities, Components, and Systems that run on events and access them as server actions, https endpoints, websocket payloads, etc. This plugin provides the WebServer as a resource in the ECS.

#### Async-Bevy-Web
This fundamental core plugin implements the traits directly on the Bevy App that power the Framework. It extends a Bevy App with various methods and structures that enable an ergonomic way to integrate the web server and ECS data transfer. It allows the user to specify initial frame rates, register the user's web application code (Leptos), etc.

### Bevy-Leptos
This fundamental core plugin allows users to define a Leptos web application and run it on the Axum server provided by the WebServer plugin. It generates the App routes, and builds the Server-Side Rendered and/or Hydrated application to manage web-based user interfaces. 

## Target API
The target API for user applications built on the framework would be something like this:

Folder Structure:
```
root/
   App/
       src/
           lib.rs <-- User Application
       Cargo.toml 
    Index/
        src/
            main.rs <-- App entry point
        Cargo.toml
    Cargo.toml // <-- Workspace definition
```
Inside of Index/src/main.rs:

```rust
use bevy::prelude::*;
use async_bevy_web::{Config, AppLoaderPlugin};
use app::App

fn main() {
    let mut app = App::new()
                        .with_default_config()
                        .with_framerate(60.0)
                        .add_plugins(AppLoaderPlugin::new(App));

    app.build().await
    app.run();
}

```

The App/lib.rs would then be:

```rust
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en"/>
        <Meta name="description" content="A static website generated using Leptos and Bevy ECS"/>
        <Stylesheet href="/pkg/cinnog_example.css"/>

        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <Routes>
                    <StaticRoute
                        path="/"
                        view=HomePage
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />

                    <StaticRoute
                        path="/404"
                        view=NotFound
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />

                    <StaticRoute
                        path="/person/*person"
                        view=HomePage
                        static_params=move || Box::pin(async move { run_system(people_static_params) })
                    />
                    
                    <StaticRoute
                        path="/blog/*post"
                        view=BlogPost
                        static_params=move || Box::pin(async move { run_system(blog_static_params) })
                    />

                </Routes>
            </main>
        </Router>
    }
}

// These functions can simply be Bevy ECS Systems that get executed with the provided run_system helper function
fn people_static_params(people: Query<&FileName, With<PersonName>>) -> StaticParamsMap {
    let mut map = StaticParamsMap::default();
    map.insert(
        "person".to_string(),
        people.iter().map(|person| person.0.clone()).collect(),
    );
    map
}

fn blog_static_params(posts: Query<&FileName, With<Post>>) -> StaticParamsMap {
    let mut map = StaticParamsMap::default();
    map.insert(
        "post".to_string(),
        posts.iter().map(|post| post.0.clone()).collect(),
    );
    map
}

#[derive(Resource, Clone)]
pub struct SiteName(pub String);
```