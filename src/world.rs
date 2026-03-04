use std::any::TypeId;
use std::collections::HashMap;
use crate::{impl_spawnable, impl_query};
use crate::components::{AnyStorage, Component, ComponentStorage, Entity};
use crate::query::{QueryData, QueryIter};
use crate::spawnable::Spawnable;
use crate::system::{AnySystem, IntoSystem, Process};

pub struct World {
    entities: Vec<Entity>,
    component_storages: HashMap<TypeId, Box<dyn AnyStorage>>,
    startup_systems: Vec<Box<dyn AnySystem>>,
    update_systems: Vec<Box<dyn AnySystem>>,
}

impl World {
    pub fn new() -> Self {
        World {
            entities: vec![],
            component_storages: HashMap::new(),
            startup_systems: vec![],
            update_systems: vec![]
        }
    }

    fn get_storage<C: Component>(&self) -> Option<&ComponentStorage<C>> {
        let key = TypeId::of::<C>();

        self.component_storages
            .get(&key)?
            .as_any()
            .downcast_ref::<ComponentStorage<C>>()
    }

    fn get_storage_mut<C: Component>(&mut self) -> &mut ComponentStorage<C> {
        let key = TypeId::of::<C>();

        self.component_storages
            .entry(key)
            .or_insert(
                Box::new(ComponentStorage::<C>::new())
            )
            .as_any_mut()
            .downcast_mut::<ComponentStorage<C>>()
            .unwrap()
    }

    fn add_component<C: Component>(&mut self, entity: Entity, component: C) {
        self.get_storage_mut::<C>().add_entity(entity, component);
    }

    pub fn spawn<S: Spawnable>(&mut self, components: S) -> Entity {
        let entity = Entity::new(self.entities.len());
        components.spawn(self, entity);

        self.entities.push(entity);
        entity
    }

    pub fn query<Q: QueryData>(&mut self) -> QueryIter<'_, Q> {
        QueryIter::new(
            self, self.entities.len()
        )
    }

    pub fn add_system<F, T>(&mut self, process: Process, function: F)
    where
        F: IntoSystem<T> + 'static,
        F::System: 'static
    {
        let system = function.into_system();
        match process {
            Process::Startup => self.startup_systems.push(Box::new(system)),
            Process::Update => self.update_systems.push(Box::new(system)),
        }
    }

    fn run_startups(&mut self) {
        let systems = std::mem::take(&mut self.startup_systems);

        for system in systems.into_iter() {
            system.run(self);
        }
    }

    pub fn run_updates(&mut self) {
        let systems = std::mem::take(&mut self.update_systems);

        for system in systems.iter() {
            system.run(self);
        }
        self.update_systems = systems;
    }

    fn create_utils(&mut self) {
    }

    pub fn start(&mut self) {
        self.create_utils();
        self.run_startups();
    }
}

impl<C: Component> Spawnable for C {
    fn spawn(self, world: &mut World, entity: Entity) {
        let component = self;
        world.add_component(entity, component);
    }
}

impl QueryData for Entity {
    type Item<'world> = Entity;
    fn fetch(_world: &mut World, entity: Entity) -> Option<Self::Item<'_>> {
        Some(entity)
    }
}

impl<C: Component> QueryData for &C {
    type Item<'world> = &'world C;
    fn fetch(world: &mut World, entity: Entity) -> Option<Self::Item<'_>> {
        world.get_storage::<C>()?.get(entity)
    }
}

impl<C: Component> QueryData for &mut C {
    type Item<'world> = &'world mut C;
    fn fetch(world: &mut World, entity: Entity) -> Option<Self::Item<'_>> {
        world.get_storage_mut::<C>().get_mut(entity)
    }
}

impl_spawnable!(A, B);
impl_spawnable!(A, B, C);
impl_spawnable!(A, B, C, D);
impl_spawnable!(A, B, C, D, E);
impl_spawnable!(A, B, C, D, E, F);
impl_spawnable!(A, B, C, D, E, F, G);
impl_spawnable!(A, B, C, D, E, F, G, H);

impl_query!(A, B);
impl_query!(A, B, C);
impl_query!(A, B, C, D);
impl_query!(A, B, C, D, E);
impl_query!(A, B, C, D, E, F);
impl_query!(A, B, C, D, E, F, G);
impl_query!(A, B, C, D, E, F, G, H);
