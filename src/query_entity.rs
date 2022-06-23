use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    rc::Rc, collections::HashMap,
};

type Component = Rc<RefCell<dyn Any + 'static>>;

type ExtractedComponents<'a> = Result<&'a Vec<Option<Rc<RefCell<dyn Any>>>>, &'static str>;

/// Helper struct made for iterating over entities with [crate::wgtr::Query::run_entity()].
/// 
/// Example:
/// ```
/// use wgtr_ecs::*; // for macros
/// let mut world = wgtr::World::new();
///
/// world.register_component::<u32>();
/// world.register_component::<f32>();
///
/// world.create_entity()
///     .with_component(10u32).unwrap()
///     .with_component(10.1f32).unwrap();
///
/// let mut query = world.query();
/// make_query!(query, u32, f32);
/// 
/// for entity in query.run_entity(){ // entity is crate::wgtr::QueryEntity
///     let mut f = get_component!(entity, &mut f32);
///     *f += 1.0;
/// }
///
/// ```
pub struct QueryEntity<'a> {
    pub id: usize,
    components: &'a HashMap<TypeId, Vec<Option<Component>>>,
}

impl<'a> QueryEntity<'a> {
    pub fn new(id: usize, components: &'a HashMap<TypeId, Vec<Option<Component>>>) -> Self {
        Self { id, components }
    }

    fn extract_components<T: Any>(&self) -> ExtractedComponents {
        let type_id = TypeId::of::<T>();
        self
            .components
            .get(&type_id)
            .ok_or_else(|| "Attempting to use not registered component")
    }

    pub fn get_component<T: Any>(&self) -> Result<Ref<T>, &'static str> {
        let components = self.extract_components::<T>()?;
        let borrowed_component = components[self.id]
            .as_ref()
            .ok_or_else(||"Attempting to get component from entity that does not have one")?
            .borrow();
        Ok(Ref::map(borrowed_component, |any| {
            any.downcast_ref::<T>().unwrap()
        }))
    }

    pub fn get_component_mut<T: Any>(&self) -> Result<RefMut<T>, &'static str> {
        let components = self.extract_components::<T>()?;
        let borrowed_component = components[self.id]
            .as_ref()
            .ok_or_else(||"Attempting to get component from entity that does not have one")?
            .borrow_mut();
        Ok(RefMut::map(borrowed_component, |any| {
            any.downcast_mut::<T>().unwrap()
        }))
    }
}
