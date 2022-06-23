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