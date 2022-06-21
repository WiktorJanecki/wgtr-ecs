use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    rc::Rc, collections::HashMap,
};

use crate::wgtr::Component;

type ExtractedComponents<'a> = Result<&'a Vec<Option<Rc<RefCell<dyn Any>>>>, &'static str>;

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
