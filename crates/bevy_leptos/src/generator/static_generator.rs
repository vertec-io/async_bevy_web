use bevy_ecs::prelude::{Res, ResMut};
// use bevy_tokio_tasks::TokioTasksRuntime;

use leptos::*;
// use leptos_axum::{generate_route_list_with_exclusions, LeptosRoutes};

use std::sync::{Arc, Mutex};
use crate::{LeptosApp, LeptosView};

pub fn start_leptos_app<F>(
    // runtime: ResMut<TokioTasksRuntime>, 
    leptos_app:Res<LeptosApp<F>>,
)
where
    F: LeptosView +'static + Clone
{

    let leptos_app_clone = Arc::new(leptos_app.clone());
    
    // runtime.spawn_background_task(|ctx| async move {
    //     println!("Generating static leptos app...")
    // });
}