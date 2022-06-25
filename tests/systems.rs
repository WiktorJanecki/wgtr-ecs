use wgtr_ecs::{System, World, Systems};

#[test]
fn create_and_init_system(){
    struct SimpleSystem{}

    impl System for SimpleSystem{
        fn init(&mut self, world : &mut wgtr_ecs::World){
            let x = world.get_resource_mut::<u32>().unwrap();
            *x += 1;
        }
    }

    let mut world = World::new();
    world.add_resource(10_u32);
    let mut systems = Systems::new();
    systems
        .with_system(SimpleSystem{})
        .with_system(SimpleSystem{});
    systems.init(&mut world);
    let new_x = world.get_resource::<u32>().unwrap();
    assert_eq!(*new_x, 12);
}

