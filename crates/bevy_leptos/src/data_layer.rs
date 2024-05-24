#[cfg(feature="generator")]
use std::any::type_name;

#[cfg(feature="generator")]
use bevy_ecs::system::{IntoSystem, Resource};

#[cfg(feature="generator")]
use leptos::expect_context;
#[cfg(feature="generator")]
pub use crate::generator::dyn_generator::AppState;

#[cfg(feature="generator")]
pub async fn use_resource<R: Resource + Clone> () ->Option<R>{

    let mut state = expect_context::<AppState>();
    state.world_context.run_on_main_thread(move |ctx| {
        ctx.world.get_resource::<R>().cloned()
    }).await
}

#[cfg(feature="generator")]
pub async fn expect_resource<R: Resource + Clone>() -> R {
    use_resource::<R>().await.unwrap_or_else(||{
        panic!("Expected resource {}, but it didn't exist", type_name::<R>())
    })
}

#[cfg(feature="generator")]
pub async fn run_system_with_input<S, I, R, T>(system: S, input: I) -> R
where
    S: Send + IntoSystem<I, R, T> + 'static,
    R: Send + 'static,
    I: Send + 'static
{
    use crate::DataLayer;
    let mut state = expect_context::<AppState>();
    state.world_context.run_on_main_thread(move |ctx| {
        ctx.world.run_user_system(system, input)
    }).await
}

#[cfg(feature="generator")]
pub async fn run_system<S, R, T>(system: S) -> R
where
    S: IntoSystem<(), R, T> + Send + 'static,
    R: Send + 'static,
{
    run_system_with_input(system, ()).await
}

