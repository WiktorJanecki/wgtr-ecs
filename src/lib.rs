pub mod wgtr{
    use std::{any::{Any, TypeId}, collections::HashMap, cell::RefCell, rc::Rc};

    #[derive(Default)]
    pub struct World{
        resources: HashMap<TypeId, Box<dyn Any>>,
        components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any + 'static>>>>>,
    }


    impl World{
        pub fn new() -> Self{
            Self::default()
        }

        pub fn register_component<T: Any>(&mut self){
            let type_id = TypeId::of::<T>();
            self.components.insert(type_id, vec![]);
        }

        pub fn create_entity(&mut self) -> &mut Self{
            self.components.iter_mut().for_each(|(_key, components)| components.push(None));
            self
        }

        pub fn with_component(&mut self, data: impl Any) -> Result<&mut Self, &'static str>{
            let type_id = data.type_id();

            if let Some(components) = self.components.get_mut(&type_id){
                let last_component = components.last_mut()
                    .ok_or_else(|| "Could not find the last component")
                    .unwrap();
                    *last_component = Some(Rc::new(RefCell::new(data)));
            }
            else{
                return Err("Tried to use with_comopnent with a component that hasnt been registered");
            }
            Ok(self)
        }



        pub fn add_resource(&mut self, resource_data: impl Any){
            let type_id = resource_data.type_id();
            self.resources.insert(type_id, Box::new(resource_data));
        }

        pub fn get_resource<T: Any>(&self) -> Option<&T>{
            let type_id = TypeId::of::<T>();
            if let Some(data) = self.resources.get(&type_id){
                return data.downcast_ref();
            }
            None
        }

        pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T>{
            let type_id = TypeId::of::<T>();
            if let Some(data) = self.resources.get_mut(&type_id){
                return data.downcast_mut();
            }
            None
        }  

        pub fn remove_resource<T: Any>(&mut self){
            let type_id = TypeId::of::<T>();
            self.resources.remove(&type_id);
        }
    }
}