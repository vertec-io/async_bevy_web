use std::any::type_name;
use std::path::Path;
use std::sync::{Arc, Mutex};

use bevy_ecs::world::World;
use bevy_ecs::{bundle::Bundle, component::Component};
use bevy_ecs::system::{BoxedSystem, EntityCommands, IntoSystem, Resource};
use bevy_ecs::world::EntityWorldMut;

use bevy_tokio_tasks::TaskContext;
use leptos::expect_context;
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
pub async fn use_resource<R: Resource + Clone> () -> Option<R>{
    let ctx = expect_context::<Arc<Mutex<TaskContext>>>();
    let mut ctx = ctx.lock().unwrap();
    ctx.run_on_main_thread(move |ctx| {
        ctx.world.get_resource::<R>().cloned()
    }).await
}
pub async fn expect_resource<R: Resource + Clone>() -> R {
    use_resource::<R>().await.unwrap_or_else(||{
        panic!("Expected resource {}, but it didn't exist", type_name::<R>())
    })
}

pub async fn run_system_with_input<S, I, R, T>(system: S, input: I) -> R
where
    S: Send + IntoSystem<I, R, T> + 'static,
    R: Send + 'static,
    I: Send + 'static
{
    let ctx = expect_context::<Arc<Mutex<TaskContext>>>();
    let mut ctx = ctx.lock().unwrap();
    ctx.run_on_main_thread(move |ctx| {
        ctx.world.run_user_system(system, input)
    }).await
}

pub async fn run_system<S, R, T>(system: S) -> R
where
    S: IntoSystem<(), R, T> + Send + 'static,
    R: Send + 'static,
{
    run_system_with_input(system, ()).await
}

#[derive(Component, Clone)]
pub struct FileName(pub String);

#[derive(Component, Clone)]
pub struct FilePath(pub String);

pub trait Ingest {
    fn ingest(self, commands: &mut EntityCommands)
    where
        Self: Sized;

    fn ingest_path(&self, commands: &mut EntityCommands, path: &Path) {
        commands.insert(default_bundle_from_path(path));
    }
}

pub fn default_bundle_from_path(path: &Path) -> impl Bundle {
    let path_string = path.to_string_lossy().into_owned();
    let file_ending = path.extension();
    let file_name = if let Some(ending) = file_ending {
        path.file_name().map(|name| {
            name.to_string_lossy()
                .trim_end_matches(&format!(".{}", ending.to_string_lossy().as_ref()))
                .to_owned()
        })
    } else {
        path.file_name()
            .map(|name| name.to_string_lossy().into_owned())
    };

    (
        FileName(file_name.expect("No file name in path")),
        FilePath(path_string),
    )
}