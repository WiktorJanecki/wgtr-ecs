use std::{any::Any, cell::{Ref, RefMut}};

use  wgtr_ecs::{*, wgtr::QueryEntity};
#[test]
fn playground() -> Result<(), &'static str>{
    let mut world = wgtr::World::new();

    world.register_component::<u32>();
    world.register_component::<f32>();

    world.create_entity()
        .with_component(10u32)?
        .with_component(10.1f32)?;
    world.create_entity()
        .with_component(2u32)?
        .with_component(210.1f32)?;
    world.create_entity()
        .with_component(3u32)?;

    // let query = world.query()
    //     .with_component::<u32>()?
    //     .with_component::<f32>()?
    //     .run();
    // let indexes = query.0;
    // let bu32s: Vec<Ref<dyn Any>> = query.1[0].iter().map(|e| e.borrow()).collect();
    // let u32s: Vec<&u32> = bu32s.iter().map(|e| e.downcast_ref::<u32>().unwrap()).collect();
    // let bf32s: Vec<Ref<dyn Any>> = query.1[1].iter().map(|e| e.borrow()).collect();
    // let f32s: Vec<&f32> = bf32s.iter().map(|e| e.downcast_ref::<f32>().unwrap()).collect();
    
    // use itertools::izip;

    // let results = izip!(indexes, u32s, f32s).into_iter();


    // for (index, u, f) in results{
    //     dbg!(index, u, f);
    // } 

    let mut query = world.query();

    let entities = query
        .with_component::<u32>()?
        .with_component::<f32>()?
        .run_entity();
    let indexes = entities.iter().map(|e| e.id);
    let u32s = entities.iter().map(|e| e.get_component::<u32>().unwrap());
    let f32s = entities.iter().map(|e| e.get_component_mut::<f32>().unwrap());

    use itertools::izip;

    for (index, u, mut f) in izip!(indexes, u32s, f32s).into_iter(){
        *f += 1.0;
        dbg!(index, u, f);
    }

    // how it should be looking
    // for (index, u, f) in query!(world, &u32,&mut f32).into_iter(){
    //     *f += 1.0;
    //     dbg!(index, u, f);
    // }

    Ok(())
}

#[macro_export]
macro_rules! query {
    ($world:ident, $($tail:tt)*) => {
        {
            let world = &mut $world;
            let mut q = world.query();
            reg_query!(q; $($tail)*);
            q.run()
        }
    };
}
#[macro_export]
macro_rules! reg_query {
    ($q:ident) => {};

    ($q:ident; & $x:ty ) => {
        let mut q = &mut $q;
        q.with_component::<$x>().unwrap();
        reg_query!(q);
    };
    ($q:ident; &mut $x:ty ) => {
        let mut q = &mut $q;
        q.with_component::<$x>().unwrap();
        reg_query!(q);
    };
    ($q:ident; & $x:ty , $($tail:tt)*) => {
        let mut q = &mut $q;
        q.with_component::<$x>().unwrap();
        reg_query!(q; $($tail)*);
    };
    ($q:ident; &mut $x:ty ,$($tail:tt)* ) => {
        let mut q = &mut $q;
        q.with_component::<$x>().unwrap();
        reg_query!(q; $($tail)*);
    };
}