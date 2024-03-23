# Async Bevy Web

Welcome to the Async Bevy Web application! This project is designed to showcase the integration of Bevy, a data-driven game engine built in Rust, with asynchronous web server capabilities using Axum. It demonstrates how to set up a basic web server that communicates with clients through WebSockets, all within the context of a Bevy application.

This application is in very early development and will change significantly over time. This ReadMe may become outdates as the project progresses. Bear with us while we build!

## Purpose

The Async Bevy Web application serves as an example of how to integrate Bevy with Axum for creating real-time, interactive web applications. This setup allows for the development of games or interactive applications that require real-time communication between the server and the client, leveraging the power and performance of Rust.

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
3. Create a `.env` file at the root directory of the project and add the following lines:
   ```
   HOST=127.0.0.1
   PORT=8000
   SERVER_NAME=Bevy Axum Server
   ```
   This configuration sets up the local server address and port.

### Running the Application

1. Build and run the application using Cargo:
   ```sh
   cargo run
   ```
   Note: The first build might take a significant amount of time as it compiles the dependencies.

2. Once the build is complete and the server is running, open the `index.html` file located in the root directory of the project in a web browser.

3. In the WebSocket address input field (upper right corner of the page), type the WebSocket address: `ws://127.0.0.1:8000/ws` and connect.

4. You should now be able to see incoming messages from the server displayed on the page.

## Exploring the Code

The project is structured into multiple crates to separate concerns between the Bevy application, the web server, and core ECS (Entity Component System) logic. Here are some key parts of the repository:

- [`crates/bevy_tokio_tasks`](https://github.com/vertec-io/async_bevy_web/tree/main/crates/bevy_tokio_tasks): Contains examples and the implementation for integrating Bevy with Tokio tasks.
- [`crates/web_server`](https://github.com/vertec-io/async_bevy_web/tree/main/crates/web_server): Implements the web server logic using Axum, including WebSocket communication.
- [`index.html`](https://github.com/vertec-io/async_bevy_web/blob/main/index.html): The client-side HTML file for connecting to the WebSocket server.

Feel free to explore the code and experiment with it to better understand how Bevy and Axum can be used together for real-time web applications.

