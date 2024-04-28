// #[cfg(feature="generator")]
// pub mod leptos_app;
// pub use crate::leptos_app::*;

#[cfg(feature="generator")]
pub mod server;
#[cfg(feature="generator")]
pub use crate::server::*;