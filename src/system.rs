use crate::World;

pub trait System{
    fn init(&mut self, _world : &mut World){}
    fn update(&mut self, _world :&mut World){}
    fn render(&mut self, _world :&mut World){}
}

#[derive(Default)]
pub struct Systems<'a>{
    pub systems: Vec<Box<dyn 'a + System>>,
}

impl <'a>Systems<'a>{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn with_system(&mut self, system:impl 'a + System) -> &mut Self{
        self.systems.push(Box::new(system));
        self
    }
    
    pub fn init(&mut self, world: &mut World){
        for system in self.systems.iter_mut(){
            system.init(world);
        }
    }

    pub fn update(&mut self, world: &mut World){
        for system in self.systems.iter_mut(){
            system.update(world);
        }
    }

    pub fn render(&mut self, world: &mut World){
        for system in self.systems.iter_mut(){
            system.render(world);
        }
    }
}
