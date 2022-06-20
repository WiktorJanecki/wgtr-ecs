use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

pub struct Query<'a> {
    map: u128,

    components_bit_masks: &'a HashMap<TypeId, u128>,
    type_ids: Vec<TypeId>,
    entities_bit_maps: &'a Vec<u128>,
    components: &'a HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any + 'static>>>>>,
}

impl<'a> Query<'a> {
    pub fn new(
        components_bit_masks: &'a HashMap<TypeId, u128>,
        entities_bit_maps: &'a Vec<u128>,
        components: &'a HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any + 'static>>>>>,
    ) -> Self {
        Self {
            map: 0,
            components_bit_masks,
            components,
            type_ids: vec![],
            entities_bit_maps,
        }
    }

    pub fn with_component<T: Any>(&mut self) -> Result<&mut Self, &'static str> {
        let type_id = TypeId::of::<T>();
        let component_mask = self
            .components_bit_masks
            .get(&type_id)
            .ok_or("Tried to query component that was not registered")?;
        self.map |= *component_mask;
        self.type_ids.push(type_id);
        Ok(self)
    }

    pub fn run(&self) -> Vec<Vec<Rc<RefCell<dyn Any>>>> {
        let indexes: Vec<usize> = self
            .entities_bit_maps
            .iter()
            .enumerate()
            .filter_map(|(index, entity_map)| {
                if entity_map & self.map == self.map {
                    return Some(index);
                }
                None
            })
            .collect();
        let mut result = vec![];

        for type_id in &self.type_ids {
            let entity_components = self.components.get(type_id).unwrap();
            let mut components_to_keep = vec![];
            for index in &indexes {
                components_to_keep.push(entity_components[*index].as_ref().unwrap().clone());
            }
            result.push(components_to_keep);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use std::any::TypeId;

    use crate::wgtr::World;

    #[test]
    fn query_mask_updating_with_component() -> Result<(), &'static str> {
        let mut world = World::new();
        world.register_component::<u32>();
        world.register_component::<f32>();
        let mut query = world.query();
        query.with_component::<u32>()?.with_component::<f32>()?;

        assert_eq!(query.map, 3);
        assert_eq!(TypeId::of::<u32>(), query.type_ids[0]);
        assert_eq!(TypeId::of::<f32>(), query.type_ids[1]);

        Ok(())
    }
    #[test]
    fn run_query() -> Result<(), &'static str> {
        let mut world = World::new();
        world.register_component::<u32>();
        world.register_component::<f32>();

        world.create_entity().with_component(5_u32)?;
        world
            .create_entity()
            .with_component(420_u32)?
            .with_component(11.1_f32)?;
        world.create_entity().with_component(0.0_f32)?;

        let mut query = world.query();
        query.with_component::<u32>()?.with_component::<f32>()?;
        let query_result = query.run();

        let u32s = &query_result[0];
        let f32s = &query_result[1];
        assert_eq!(u32s.len(), 1);
        assert_eq!(f32s.len(), 1);

        let first_u32 = u32s[0].borrow();
        let extracted_u32 = first_u32.downcast_ref::<u32>().unwrap();
        assert_eq!(*extracted_u32, 420);

        Ok(())
    }
}
