use crate::components::Entity;
use crate::world::World;

pub trait Spawnable {
    fn spawn(self, world: &mut World, entity: Entity);
}

impl Spawnable for () {
    fn spawn(self, _world: &mut World, _entity: Entity) { }
}

#[macro_export]
macro_rules! impl_spawnable {
    ($($ty:ident),+) => {
        impl<$($ty: Component),+> Spawnable for ($($ty,)+) {
            fn spawn(self, world: &mut World, entity: Entity) {
                #[allow(non_snake_case)]
                let ($($ty,)+) = self;
                $(world.add_component(entity, $ty);)+
            }
        }
    };
}
