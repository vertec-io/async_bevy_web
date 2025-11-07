
use std::sync::Arc;
use std::pin::Pin;
use std::future::Future;
use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;

pub use abw_macros::leptos_app;
pub type LeptosAppFn = Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

#[derive(Resource, Clone)]
pub struct LeptosApp {
    pub app_fn: LeptosAppFn,
}

impl LeptosApp {
    pub fn new(app_fn: LeptosAppFn) -> Self {
        Self { app_fn }
    }
}

pub struct LeptosAppPlugin {
    leptos_app: LeptosApp,
}

impl LeptosAppPlugin {
    pub fn new(app_fn: LeptosAppFn) -> Self {
        let leptos_app = LeptosApp::new(app_fn);
        Self { leptos_app }
    }
}


impl Plugin for LeptosAppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.leptos_app.clone())
            .add_systems(PostStartup, start_leptos_app);
    }
}

pub fn start_leptos_app(
    runtime: ResMut<TokioTasksRuntime>,
    leptos_app: Res<LeptosApp>,
) {
    let app_fn = leptos_app.app_fn.clone();
    runtime.spawn_background_task(move |_| async move {
        app_fn().await;
    });
}