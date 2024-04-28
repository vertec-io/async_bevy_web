#[cfg(feature="generator")]
pub mod dyn_generator;

#[cfg(not(feature="generator"))]
pub mod static_generator;
