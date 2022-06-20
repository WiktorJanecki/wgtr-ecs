use std::{any::Any, cell::RefCell, rc::Rc};

use wgtr_ecs::wgtr::*;

#[test]
fn create_query() -> Result<(), &'static str> {
    let mut world = World::new();
    world.register_component::<Health>();
    world.register_component::<Speed>();

    world
        .create_entity()
        .with_component(Health(100))?
        .with_component(Speed(10))?;

    world.create_entity().with_component(Speed(13))?;

    world.create_entity().with_component(Health(130))?;

    world
        .create_entity()
        .with_component(Health(200))?
        .with_component(Speed(12))?;

    let query = world
        .query()
        .with_component::<Health>()?
        .with_component::<Speed>()?
        .run();
    let healths: &Vec<Rc<RefCell<dyn Any>>> = &query.1[0];
    let speeds: &Vec<Rc<RefCell<dyn Any>>> = &query.1[1];

    assert_eq!(healths.len(), speeds.len());
    assert_eq!(healths.len(), 2);

    let borrowed_first_health = healths[0].borrow();
    let first_health = borrowed_first_health.downcast_ref::<Health>().unwrap();
    assert_eq!(first_health.0, 100);
    let borrowed_first_speed = speeds[0].borrow();
    let first_speed = borrowed_first_speed.downcast_ref::<Speed>().unwrap();
    assert_eq!(first_speed.0, 10);

    let borrowed_second_health = healths[1].borrow();
    let second_healt = borrowed_second_health.downcast_ref::<Health>().unwrap();
    assert_eq!(second_healt.0, 200);
    let mut borrowed_second_speed = speeds[1].borrow_mut();
    let mut second_speed = borrowed_second_speed.downcast_mut::<Speed>().unwrap();
    second_speed.0 += 1;
    assert_eq!(second_speed.0, 13);

    Ok(())
}

struct Health(pub u32);
struct Speed(pub u32);
