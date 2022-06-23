use  wgtr_ecs::{*};
#[test]
fn macros() -> Result<(), &'static str>{
    let mut world = wgtr::World::new();

    world.register_component::<u32>();
    world.register_component::<f32>();

    world.create_entity()
        .with_component(10u32)?
        .with_component(10.1f32)?;

    let mut query = world.query();
    make_query!(query, u32, f32);
    for entity in query.run_entity(){
        let mut f = get_component!(entity, &mut f32);
        *f += 1.0;
    }
    let mut query = world.query();
    make_query!(query, u32, f32);
    for entity in query.run_entity(){
        let u = get_component!(entity, &u32);
        let f = get_component!(entity, &f32);
        assert_eq!(*u, 10);
        assert_eq!(*f, 11.1);
    }
    Ok(())
}
