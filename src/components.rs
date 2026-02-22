#[cfg(feature = "derive")]
pub use ecs_derive::Component;

use std::any::Any;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Entity {
    index: usize
}

impl Entity {
    pub fn new(index: usize) -> Self {
        Entity { index }
    }
}

pub trait Component: 'static {}

pub trait AnyStorage: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ComponentStorage<C: Component> {
    sparse: [Option<usize>; 32],
    dense: Vec<C>
}

impl<C: Component> ComponentStorage<C> {
    pub fn new() -> Self {
        ComponentStorage {
            sparse: [None; 32],
            dense: vec![]
        }
    }

    pub fn add_entity(&mut self, entity: Entity, component: C) {
        self.sparse[entity.index] = Some(self.dense.len());
        self.dense.push(component);
    }

    pub fn get(&self, entity: Entity) -> Option<&C> {
        self.dense.get(self.sparse[entity.index]?)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        self.dense.get_mut(self.sparse[entity.index]?)
    }
}

impl<C: Component> AnyStorage for ComponentStorage<C> {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}