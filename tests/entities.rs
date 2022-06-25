use wgtr_ecs::*;

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

#[test]
fn delete_component_from_entity() -> Result<(), &'static str> {
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
    world.remove_component::<Health>(0)?;
    let query = world
        .query()
        .with_component::<Health>()?
        .with_component::<Speed>()?
        .run();
    assert_eq!(query.0.len(), 1);
    assert_eq!(query.0[0], 1);
    Ok(())
}

#[test]
fn add_component_to_entity() -> Result<(), &'static str> {
    let mut world = World::new();
    world.register_component::<Health>();
    world.register_component::<Speed>();

    world.create_entity().with_component(Health(100))?;

    world.add_component(Speed(22), 0)?;

    let query = world
        .query()
        .with_component::<Health>()?
        .with_component::<Speed>()?
        .run();

    assert_eq!(query.0.len(), 1);

    Ok(())
}

#[test]
fn delete_entity() -> Result<(), &'static str> {
    let mut world = World::new();
    world.register_component::<Health>();

    world.create_entity().with_component(Health(100))?;

    world.create_entity().with_component(Health(200))?;

    let query = world.query().with_component::<Health>()?.run();

    assert_eq!(query.0.len(), 2);
    assert_eq!(query.0[0], 0);

    world.remove_entity(0)?;

    let query = world.query().with_component::<Health>()?.run();

    assert_eq!(query.0.len(), 1);
    assert_eq!(query.0[0], 1);

    Ok(())
}

struct Health(pub u32);
struct Speed(pub u32);
