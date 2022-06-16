use wgtr_ecs::*;

#[test]
fn create_and_get_resource_immutably(){
    let mut world = World::new();

    world.add_resource(FpsResource(60));

    let fps = world.get_resource::<FpsResource>().unwrap();
    assert_eq!(fps.0,60);
}
#[test]
fn create_and_get_resource_mutably(){
    let mut world = World::new();

    world.add_resource(FpsResource(60));
    {
        let fps = world.get_resource_mut::<FpsResource>().unwrap();
        fps.0 += 1;
    }
    let fps = world.get_resource::<FpsResource>().unwrap();
    assert_eq!(fps.0,61);

}
#[test]
fn remove_resource(){
    let mut world = World::new();

    world.add_resource(FpsResource(60));
    world.remove_resource::<FpsResource>();

    let removed_resource = world.get_resource::<FpsResource>();
    assert!(removed_resource.is_none());
}

struct FpsResource(pub u32);
