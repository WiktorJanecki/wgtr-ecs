/// Macro which helps with getting components when querying with [crate::Query::run_entity()]. It does unwrap an error when attempting to use invalid component for exapmle unregistered one or unqueried one.
/// We will use get_component!(entity & TYPE); for to get immutable reference and get_component!(entity &mut TYPE); to get mutable one.
/// 
/// Example:
/// ```
/// use wgtr_ecs::get_component;
/// use wgtr_ecs::*;
/// 
/// let mut world = World::new();
/// world.register_component::<u32>();
/// world.register_component::<f32>();
/// world.create_entity()
///     .with_component(100_u32).unwrap() // we registered our component before so nothing can go wrong
///     .with_component(10.0_f32).unwrap(); // although I encourage to use with_component(10.0_f32)?; and returning an result
///
/// let mut query = world.query();
/// let entities: Vec<QueryEntity> = query
///     .with_component::<u32>().unwrap() // same as before; 
///     .with_component::<f32>().unwrap()
///     .run_entity(); 
///
/// assert_eq!(entities.len(), 1);
///
/// for entity in entities {
///     assert_eq!(entity.id, 0);
///     let health: std::cell::Ref<u32> = get_component!(entity, &u32); // of course you don't have to specify the type, it is here only for clarity
///     let mut speed: std::cell::RefMut<f32> = get_component!(entity, &mut f32);
///     *speed += 2.0;
///     assert_eq!(*health, 100);
///     assert_eq!(*speed, 12.0);
/// }
/// ```
#[macro_export]
macro_rules! get_component {
    ($entity:ident, & $x:ty) => {
        || -> std::cell::Ref<$x> {
            let ent = &$entity;
            return ent.get_component::<$x>().unwrap();
        }()
    };
    ($entity:ident, &mut $x:ty) => {
        || -> std::cell::RefMut<$x> {
            let ent = &$entity;
            return ent.get_component_mut::<$x>().unwrap();
        }()
    };
}


/// Macro which helps registering all components in a query. If component was not registered before it will panic.
///
/// Example:
/// ```
/// use wgtr_ecs::*; // for macros
/// let mut world = World::new();
///
/// world.register_component::<u32>();
/// world.register_component::<f32>();
///
/// world.create_entity()
///     .with_component(10u32).unwrap()
///     .with_component(10.1f32).unwrap();
///
/// let mut query = world.query();
/// make_query!(query, u32, f32);
/// for entity in query.run_entity(){
///     let mut f = get_component!(entity, &mut f32);
///     *f += 1.0;
/// }
///
/// ```
#[macro_export]
macro_rules! make_query {
    ($q:ident) => {};
    ($q:ident,  $x:ty ) => {
        let q = &mut $q;
        q.with_component::<$x>().unwrap();
    };
    ($q:ident, $x:ty , $($tail:tt)*) => {
        let mut q = &mut $q;
        q.with_component::<$x>().unwrap();
        make_query!(q, $($tail)*);
    };
}