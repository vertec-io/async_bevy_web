
mod leptos_app;
pub use crate::leptos_app::*;

pub mod server;
pub use crate::server::*;
use std::sync::Arc;
use leptos::LeptosOptions;

#[derive(Clone)]
pub struct AppState{
    pub server: Arc<WebServer>,
    pub leptos_options: LeptosOptions,
}