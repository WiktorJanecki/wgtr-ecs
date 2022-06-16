pub mod wgtr{
    use std::{any::{Any, TypeId}, collections::HashMap};

    #[derive(Default)]
    pub struct World{
        resources: HashMap<TypeId, Box<dyn Any>>,
    }


    impl World{
        pub fn new() -> Self{
            Self::default()
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