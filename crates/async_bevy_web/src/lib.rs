mod config;
pub mod prelude;

use std::{any::type_name, path::Path, sync::{Arc, Mutex}};

use bevy::{app::App, ecs::world::World};
use bevy::ecs::{bundle::Bundle, component::Component};
use bevy::ecs::system::{BoxedSystem, EntityCommands, IntoSystem, Resource};
use bevy::ecs::world::EntityWorldMut;
use bevy_tokio_tasks::{MainThreadContext, TaskContext};
use web_server::{create_action, expect_context};

pub use crate::prelude::*;

trait DataLayer {

    fn get_resource<R: Resource + Clone>(&self) -> Option<R>;
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut;
    fn run_boxed<R: 'static, I:'static>(
        &mut self,
        system: &mut BoxedSystem<I,R>,
        input: I,
    ) -> R;
    fn run_system<S, I, R, T>(&mut self, system:S, input: I) -> R
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

    fn run_system<S, I, R, T>(&mut self, system:S, input: I) -> R
        where
            S: IntoSystem<I, R, T>,
            R: 'static,
            I: 'static
    {
        let mut boxed_system: BoxedSystem<I, R> = Box::new(IntoSystem::into_system(system));
        self.run_boxed(&mut boxed_system, input)
    }

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