use wgtr_ecs::wgtr::*;

#[test]
fn create_entity() -> Result<(), &'static str> {
    let mut world = World::new();
    world.register_component::<Health>();
    world.register_component::<Speed>();

    world
        .create_entity()
        .with_component(Health(100))?
        .with_component(Speed(10))?;
    Ok(())
}

struct Health(pub u32);
struct Speed(pub u32);
