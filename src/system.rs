use std::marker::PhantomData;
use crate::query::{Query, QueryData};
use crate::world::World;

pub trait AnySystem {
    fn run(&self, world: &mut World);
}

pub trait IntoSystem<T> {
    type System: AnySystem;
    fn into_system(self) -> Self::System;
}

#[derive(Clone)]
pub struct System1<F, Q: QueryData> {
    function: F,
    _phantom: PhantomData<Q>
}

impl<F, Q> AnySystem for System1<F, Q>
where
    Q: QueryData,
    F: Fn(Query<Q>)
{
    fn run(&self, world: &mut World) {
        let query = Query::new(world.query::<Q>());
        (self.function)(query)
    }
}

impl<F, Q: QueryData> System1<F, Q> {
    pub fn new(function: F) -> Self {
        System1 {
            function,
            _phantom: PhantomData::default()
        }
    }
}

#[derive(Clone)]
pub struct System2<F, A: QueryData, B: QueryData> {
    function: F,
    _phantom: PhantomData<(A, B)>
}

impl<F, A, B> AnySystem for System2<F, A, B>
where
    A: QueryData,
    B: QueryData,
    F: Fn(Query<A>, Query<B>)
{
    fn run(&self, world: &mut World) {
        let world: *mut World = world;
        let query_a = unsafe { Query::new((&mut *world).query::<A>()) };
        let query_b = unsafe { Query::new((&mut *world).query::<B>()) };
        (self.function)(query_a, query_b)
    }
}

impl<F, A: QueryData, B: QueryData> System2<F, A, B> {
    pub fn new(function: F) -> Self {
        System2 {
            function,
            _phantom: PhantomData::default()
        }
    }
}

#[derive(Clone)]
pub struct System3<F, A: QueryData, B: QueryData, C: QueryData> {
    function: F,
    _phantom: PhantomData<(A, B, C)>
}

impl<F, A, B, C> AnySystem for System3<F, A, B, C>
where
    A: QueryData,
    B: QueryData,
    C: QueryData,
    F: Fn(Query<A>, Query<B>, Query<C>)
{
    fn run(&self, world: &mut World) {
        let world: *mut World = world;
        let query_a = unsafe { Query::new((&mut *world).query::<A>()) };
        let query_b = unsafe { Query::new((&mut *world).query::<B>()) };
        let query_c = unsafe { Query::new((&mut *world).query::<C>()) };
        (self.function)(query_a, query_b, query_c)
    }
}

impl<F, A: QueryData, B: QueryData, C: QueryData> System3<F, A, B, C> {
    pub fn new(function: F) -> Self {
        System3 {
            function,
            _phantom: PhantomData::default()
        }
    }
}

impl<F, Q> IntoSystem<(Q,)> for F
where
    Q: QueryData + 'static,
    F: Fn(Query<Q>) + 'static
{
    type System = System1<F, Q>;
    fn into_system(self) -> Self::System {
        System1::new(self)
    }
}

impl<F, A, B> IntoSystem<(A, B)> for F
where
    A: QueryData + 'static,
    B: QueryData + 'static,
    F: Fn(Query<A>, Query<B>) + 'static
{
    type System = System2<F, A, B>;
    fn into_system(self) -> Self::System {
        System2::new(self)
    }
}

impl<F, A, B, C> IntoSystem<(A, B, C)> for F
where
    A: QueryData + 'static,
    B: QueryData + 'static,
    C: QueryData + 'static,
    F: Fn(Query<A>, Query<B>, Query<C>) + 'static
{
    type System = System3<F, A, B, C>;
    fn into_system(self) -> Self::System {
        System3::new(self)
    }
}

pub enum Process {
    Startup,
    Update
}
