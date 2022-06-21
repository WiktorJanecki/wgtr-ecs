mod query;
mod query_entity;

pub mod wgtr {
    pub use crate::query::*;
    pub use crate::query_entity::*;

    use std::{
        any::{Any, TypeId},
        cell::RefCell,
        collections::HashMap,
        rc::Rc,
    };

    pub type Component = Rc<RefCell<dyn Any + 'static>>;

    #[derive(Default)]
    pub struct World {
        resources: HashMap<TypeId, Box<dyn Any>>,
        components: HashMap<TypeId, Vec<Option<Component>>>,
        bit_masks: HashMap<TypeId, u128>, // every component has its own mask
        bit_maps: Vec<u128>, // every entity has its map which shows which components does it has

        creature_id: usize,     // id of entity that is being now created
        free_spots: Vec<usize>, // free spots to create entity after removing one
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
            if self.free_spots.len() > 0 && self.free_spots[0] != 0 {
                // if there are free spots
                let free_index = self.free_spots.last().unwrap();
                self.creature_id = *free_index;
                self.free_spots.pop();
                return self;
            }
            self.creature_id = 0;
            self.components
                .iter_mut()
                .for_each(|(_key, components)| components.push(None));
            self.bit_maps.push(0);
            self
        }

        pub fn remove_entity(&mut self, index: usize) -> Result<(), &'static str> {
            let map = self
                .bit_maps
                .get_mut(index)
                .ok_or_else(|| "Tried to remove entity that does not exist")?;
            *map = 0;
            if index != 0 {
                self.free_spots.push(index);
            }
            Ok(())
        }

        pub fn with_component(&mut self, data: impl Any) -> Result<&mut Self, &'static str> {
            let type_id = data.type_id();

            if self.creature_id != 0 {
                // if there are free spots
                let components = self.components.get_mut(&type_id).unwrap();
                let indexed_component = components.get_mut(self.creature_id).unwrap();
                *indexed_component = Some(Rc::new(RefCell::new(data)));
                let bitmask = self.bit_masks.get(&type_id).unwrap();
                self.bit_maps[self.creature_id] |= *bitmask;

                return Ok(self);
            }

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

        pub fn add_component(&mut self, data: impl Any, index: usize) -> Result<(), &'static str> {
            let type_id = data.type_id();
            let mask = self
                .bit_masks
                .get(&type_id)
                .ok_or_else(|| "Trying to add not registered component")?;

            self.bit_maps[index] |= mask;
            self.components.get_mut(&type_id).unwrap()[index] = Some(Rc::new(RefCell::new(data)));

            Ok(())
        }

        pub fn remove_component<T: Any>(&mut self, index: usize) -> Result<(), &'static str> {
            let type_id = TypeId::of::<T>();
            let mask = self
                .bit_masks
                .get(&type_id)
                .ok_or_else(|| "Tried to remove component from entity that does not have one!")?;

            if self.has_component(index, *mask) {
                self.bit_maps[index] ^= *mask;
            }

            Ok(())
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

        fn has_component(&self, index: usize, mask: u128) -> bool {
            self.bit_maps[index] & mask == mask
        }
    }

    #[cfg(test)]
    mod test {
        use std::{any::TypeId, cell::Ref};

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

        #[test]
        fn create_entity_in_free_spot() -> Result<(), &'static str> {
            let mut world = World::new();
            world.register_component::<Health>();
            world.register_component::<Speed>();

            world
                .create_entity()
                .with_component(Health(100))?
                .with_component(Speed(10))?;
            world
                .create_entity()
                .with_component(Health(100))?
                .with_component(Speed(10))?;
            world
                .create_entity()
                .with_component(Health(100))?
                .with_component(Speed(10))?;
            world
                .create_entity()
                .with_component(Health(100))?
                .with_component(Speed(10))?;
            world
                .create_entity()
                .with_component(Health(100))?
                .with_component(Speed(10))?;

            world.remove_entity(1)?;
            world.remove_entity(3)?;

            let query = world
                .query()
                .with_component::<Health>()?
                .with_component::<Speed>()?
                .run();
            assert_eq!(query.0.len(), 3);
            world.create_entity().with_component(Health(300))?;
            world
                .create_entity()
                .with_component(Health(400))?
                .with_component(Speed(10))?;
            world
                .create_entity()
                .with_component(Health(500))?
                .with_component(Speed(10))?;
            let query = world.query().with_component::<Health>()?.run();
            assert_eq!(query.0.len(), 6);
            assert_eq!(query.0[1], 1);
            let undone_healths = &query.1[0];
            let healths: Vec<Ref<dyn Any>> = undone_healths.iter().map(|e| e.borrow()).collect();
            let deref_healths: Vec<&Health> = healths
                .iter()
                .map(|e| e.downcast_ref::<Health>().unwrap())
                .collect();
            assert_eq!(deref_healths[3].0, 300); // first free slot
            assert_eq!(deref_healths[1].0, 400); // second free slot
            assert_eq!(deref_healths[5].0, 500); // no free slots
            assert_eq!(deref_healths[2].0, 100); // untouched
                                                 //assert_eq!(health.0, 300);
            Ok(())
        }
        #[derive(Debug)]
        struct Health(pub u32);
        struct Speed(pub u32);
    }
}
