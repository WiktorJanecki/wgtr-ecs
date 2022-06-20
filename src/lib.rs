mod query;

pub mod wgtr {
    pub use crate::query::*;
    use std::{
        any::{Any, TypeId},
        cell::RefCell,
        collections::HashMap,
        rc::Rc,
    };

    #[derive(Default)]
    pub struct World {
        resources: HashMap<TypeId, Box<dyn Any>>,
        components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any + 'static>>>>>,
        bit_masks: HashMap<TypeId, u128>, // every component has its own mask
        bit_maps: Vec<u128>, // every entity has its map which shows which components does it has
    }

    impl World {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn register_component<T: Any>(&mut self) {
            let type_id = TypeId::of::<T>();
            self.components.insert(type_id, vec![]);
            self.bit_masks.insert(type_id, 1 << self.bit_masks.len());
        }

        pub fn create_entity(&mut self) -> &mut Self {
            self.components
                .iter_mut()
                .for_each(|(_key, components)| components.push(None));
            self.bit_maps.push(0);
            self
        }

        pub fn with_component(&mut self, data: impl Any) -> Result<&mut Self, &'static str> {
            let type_id = data.type_id();
            let map_index = self.bit_maps.len() - 1;

            if let Some(components) = self.components.get_mut(&type_id) {
                let last_component = components
                    .last_mut()
                    .ok_or_else(|| "Could not find the last component")
                    .unwrap();
                *last_component = Some(Rc::new(RefCell::new(data)));
                let bitmask = self.bit_masks.get(&type_id).unwrap();
                self.bit_maps[map_index] |= *bitmask;
            } else {
                return Err(
                    "Tried to use with_comopnent with a component that hasnt been registered",
                );
            }
            Ok(self)
        }

        pub fn query(&self) -> Query {
            Query::new(&self.bit_masks, &self.bit_maps, &self.components)
        }

        pub fn add_resource(&mut self, resource_data: impl Any) {
            let type_id = resource_data.type_id();
            self.resources.insert(type_id, Box::new(resource_data));
        }

        pub fn get_resource<T: Any>(&self) -> Option<&T> {
            let type_id = TypeId::of::<T>();
            if let Some(data) = self.resources.get(&type_id) {
                return data.downcast_ref();
            }
            None
        }

        pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
            let type_id = TypeId::of::<T>();
            if let Some(data) = self.resources.get_mut(&type_id) {
                return data.downcast_mut();
            }
            None
        }

        pub fn remove_resource<T: Any>(&mut self) {
            let type_id = TypeId::of::<T>();
            self.resources.remove(&type_id);
        }
    }

    #[cfg(test)]
    mod test {
        use std::any::TypeId;

        use crate::wgtr::*;

        #[test]
        fn register_component() {
            let mut world = World::new();
            world.register_component::<Health>();
            let type_id = TypeId::of::<Health>();
            let health_components = world.components.get(&type_id).unwrap();
            assert_eq!(health_components.len(), 0);
        }

        #[test]
        fn bitmask_updated_when_registering_component() {
            let mut world = World::new();

            world.register_component::<Health>();
            let type_id = TypeId::of::<Health>();
            let mask = world.bit_masks.get(&type_id).unwrap();
            assert_eq!(*mask, 1);

            world.register_component::<Speed>();
            let type_id = TypeId::of::<Speed>();
            let mask = world.bit_masks.get(&type_id).unwrap();
            assert_eq!(*mask, 2);
        }

        #[test]
        fn bitmap_is_updated_when_creating_entities() -> Result<(), &'static str> {
            let mut world = World::new();
            world.register_component::<Health>();
            world.register_component::<Speed>();

            world
                .create_entity()
                .with_component(Health(100))?
                .with_component(Speed(10))?;

            let entity_map = world.bit_maps[0];
            assert_eq!(entity_map, 3); // 0b001 + 0b010 = 0b011 = 1 + 2 = 3

            world.create_entity().with_component(Speed(10))?;

            let entity_map = world.bit_maps[1];
            assert_eq!(entity_map, 2);

            Ok(())
        }
        struct Health(pub u32);
        struct Speed(pub u32);
    }
}
