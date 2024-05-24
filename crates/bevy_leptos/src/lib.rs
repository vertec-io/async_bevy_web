use leptos::IntoView;
use bevy_app::{App, Plugin, PostStartup};
use bevy_ecs::bundle::Bundle;
use bevy_ecs::system::{BoxedSystem, IntoSystem, Resource};
use bevy_ecs::world::{EntityWorldMut, World};

mod generator;
pub mod data_layer;

#[cfg(feature="generator")]
use generator::dyn_generator::start_leptos_app;

#[cfg(not(feature="generator"))]
use generator::static_generator::start_leptos_app;

#[cfg(feature="generator")]
pub mod server;
// #[cfg(feature="generator")]
// use server::WebServerPlugin;

#[cfg(feature="generator")]
pub use server::*;

pub trait LeptosView: IntoView + Send + Sync + 'static + Clone +Copy {}
impl<T> LeptosView for T where T: IntoView + Send + Sync + 'static + Clone +Copy {}

#[derive(Resource, Debug, Clone, Copy)]
pub struct LeptosApp<F: LeptosView> {
    pub app_fn: F
}

impl<F> LeptosApp <F>
where
    F: LeptosView,
{
    pub fn new(app_fn:F) -> Self {
        Self {app_fn}
    }
}

pub struct LeptosAppPlugin<F>
where
    F: LeptosView
{
    leptos_app: LeptosApp<F>,
}

impl<F> LeptosAppPlugin <F>
where
    F: LeptosView
{
    pub fn new(app_fn: F) -> Self {
        let leptos_app = LeptosApp{app_fn};
        Self {
            leptos_app
        }
    }
}

impl <F> Plugin for LeptosAppPlugin<F>
where
    F: LeptosView
{
    fn build(&self, app: &mut App) {
        app.insert_resource(self.leptos_app.clone())
            .add_systems(PostStartup, start_leptos_app::<F>);

        #[cfg(feature="generator")]
        app.add_plugins(WebServerPlugin);
    }
}

trait DataLayer {

    fn get_resource<R: Resource + Clone>(&self) -> Option<R>;
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut;
    fn run_boxed<R: 'static, I:'static>(
        &mut self,
        system: &mut BoxedSystem<I,R>,
        input: I,
    ) -> R;
    fn run_user_system<S, I, R, T>(&mut self, system:S, input: I) -> R
    where
        S: IntoSystem<I, R, T>,
        R: 'static,
        I: 'static;
}

impl DataLayer for World {

    fn get_resource<R: Resource + Clone>(&self) -> Option<R> {
        self.get_resource::<R>().cloned()
    }

    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut {
        self.spawn(bundle)
    }

    fn run_boxed<R: 'static, I:'static>(
        &mut self,
        system: &mut BoxedSystem<I,R>,
        input: I,
    ) -> R {
        system.initialize( self);
        let to_return = system.run(input,  self);
        system.apply_deferred( self);
        to_return
    }

    fn run_user_system<S, I, R, T>(&mut self, system:S, input: I) -> R
        where
            S: IntoSystem<I, R, T>,
            R: 'static,
            I: 'static
    {
        let mut boxed_system: BoxedSystem<I, R> = Box::new(IntoSystem::into_system(system));
        self.run_boxed(&mut boxed_system, input)
    }

}

