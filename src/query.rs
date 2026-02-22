use std::marker::PhantomData;
use crate::components::Entity;
use crate::world::World;

pub struct Query<'world, Q: QueryData> {
    query_iter: QueryIter<'world, Q>
}

impl<'world, Q: QueryData> Query<'world, Q> {
    pub fn new(query_iter: QueryIter<'world, Q>) -> Self {
        Query { query_iter }
    }
}

impl<'world, Q: QueryData> Iterator for Query<'world, Q> {
    type Item = Q::Item<'world>;
    fn next(&mut self) -> Option<Self::Item> {
        self.query_iter.next()
    }
}

pub trait QueryData {
    type Item<'world>;
    fn fetch(world: &mut World, entity: Entity) -> Option<Self::Item<'_>>;
}

pub struct QueryIter<'world, Q: QueryData> {
    world: &'world mut World,
    current: usize,
    entities_count: usize,
    _phantom: PhantomData<Q>
}

impl<'world, Q: QueryData> QueryIter<'world, Q> {
    pub fn new(world: &'world mut World, entities_count: usize) -> Self {
        QueryIter {
            world, entities_count,
            current: 0,
            _phantom: PhantomData::default()
        }
    }
}

impl<'world, Q: QueryData> Iterator for QueryIter<'world, Q> {
    type Item = Q::Item<'world>;
    fn next(&mut self) -> Option<Self::Item> {
        let world: *mut World = self.world;
        while self.current < self.entities_count {
            let entity = Entity::new(self.current);
            let world = unsafe { &mut *world };
            if let Some(item) = Q::fetch(world, entity) {
                self.current += 1;
                return Some(item)
            }
            self.current += 1;
        }
        None
    }
}

#[macro_export]
macro_rules! impl_query {
    ($($ty:ident),+) => {
        impl<$($ty: QueryData),+> QueryData for ($($ty,)+)
        {
            type Item<'world> = ($($ty::Item<'world>,)+);
            fn fetch<'world>(world: &'world mut World, entity: Entity) -> Option<Self::Item<'world>> {
                let world: *mut World = world;

                $(
                        #[allow(non_snake_case)]
                        let $ty = unsafe {
                            $ty::fetch(&mut *world, entity)?
                        };
                )*

                Some(($($ty,)+))
            }
        }
    };
}