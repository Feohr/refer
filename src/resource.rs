use std::collections::HashMap;
use std::any::{Any, TypeId};

#[derive(Default)]
pub struct Resource {
    _inner: HashMap<TypeId, Box<dyn Any>>,
}

impl Resource {
    pub fn insert<T: 'static>(&mut self, item: T) {
        let id = std::any::TypeId::of::<T>();
        if self._inner.get(&id).is_some() {
            panic!("Resource of type {} is already present", std::any::type_name::<T>());
        }
        self._inner.insert(id, Box::new(item));
    }

    pub fn get<'a, T: 'static>(&'a self) -> &'a T {
        let id = std::any::TypeId::of::<T>();

        let type_nm = std::any::type_name::<T>();

        self
            ._inner
            .get(&id)
            .expect(&format!("There is no resource {type_nm}"))
            .downcast_ref::<T>()
            .expect("Error while downcasting to &{type_nm}")
    }

    pub fn get_mut<'a, T: 'static>(&'a mut self) -> &'a mut T {
        let id = std::any::TypeId::of::<T>();

        let type_nm = std::any::type_name::<T>();

        self
            ._inner
            .get_mut(&id)
            .expect(&format!("There is no resource {type_nm}"))
            .downcast_mut::<T>()
            .expect("Error while downcasting to &{type_nm}")
    }
}
