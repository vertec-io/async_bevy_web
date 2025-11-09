# Async Bevy Web

Welcome to Async Bevy Web - This project is designed to showcase the integration of Bevy, a data-driven game engine built in Rust, with asynchronous web server capabilities using Axum and Leptos. It demonstrates how to set up a basic web server that communicates with clients through HTTP and WebSockets, all within the context of a Bevy application serving as the data layer.

This application is in very early development and will change significantly over time. This ReadMe may become outdates as the project progresses.

## Purpose

Async Bevy Web provides several plugins for Bevy for creating real-time, interactive web applications. This setup allows for the development of games or interactive applications that require handling complex data on the server and providing real-time communication/updates between the server and the client, leveraging the power and performance of Rust.

## Version Compatibility

| async-bevy-web version | bevy version | bevy-tokio-tasks version | bevy-leptos version |
|---|---|---|---|
| 0.3.0 | 0.16.0 | 0.16.0 | 0.1.0 |

## Getting Started

To get the application running on your local machine, follow these steps:

### Prerequisites

- Ensure you have Rust and Cargo installed on your machine. If not, you can install them by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Setup

1. Clone the repository to your local machine:
   ```sh
   git clone https://github.com/vertec-io/async_bevy_web.git
   ```
2. Navigate into the cloned repository directory:
   ```sh
   cd async_bevy_web
   ```
3. TODO

### Running the Application

TODO

## Exploring the Code

The project is structured into multiple crates to separate concerns between the Bevy application, the web server, and core ECS (Entity Component System) logic. Here are some key parts of the repository:

- [`crates/bevy_tokio_tasks`](https://github.com/vertec-io/async_bevy_web/tree/main/crates/bevy_tokio_tasks): Contains examples and the implementation for integrating Bevy with Tokio tasks.
- [`crates/bevy_leptos`](https://github.com/vertec-io/async_bevy_web/tree/main/crates/web_server): Implements the web server logic using Axum, including WebSocket communication.
<!-- - [`index.html`](https://github.com/vertec-io/async_bevy_web/blob/main/index.html): The client-side HTML file for connecting to the WebSocket server. -->

Feel free to explore the code and experiment with it to better understand how Bevy and Axum can be used together for real-time web applications.

