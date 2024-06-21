use std::collections::HashMap;
use std::any::{Any, TypeId};

use uuid::Uuid;

#[derive(Default)]
pub struct Resource {
    _inner: HashMap<Uuid, HashMap<TypeId, Box<dyn Any>>>,
}

pub trait GetUuid {
    fn uuid() -> Uuid;
}

impl<T> GetUuid for T {
    fn uuid() -> Uuid {
        let bytes = arr_to_exact(std::any::type_name::<T>().as_bytes());
        Uuid::new_v8(bytes)
    }
}

fn arr_to_exact<T: Copy + Default, const N: usize>(array: &[T]) -> [T; N] {
    let mut ret = [T::default(); N];
    for index in 0..N {
        if let Some(x) = array.get(index) {
            ret[index] = *x;
        }
    }
    ret
}

impl Resource {
    pub fn insert<S: GetUuid, T: 'static>(&mut self, item: T) {
        let uuid = S::uuid();

        match self._inner.get_mut(&uuid) {
            Some(ref mut inner) => {
                let id = std::any::TypeId::of::<T>();
                if inner.get_mut(&id).is_some() {
                    panic!("Resource of type {} is already present", std::any::type_name::<T>());
                }
                inner.insert(id, Box::new(item));
            },
            None => {
                let mut inner = HashMap::<TypeId, Box<dyn Any>>::new();
                inner.insert(std::any::TypeId::of::<T>(), Box::new(item));
                self._inner.insert(uuid, inner);
            }
        }
    }

    pub fn get<'a, S: GetUuid, T: 'static>(&'a self) -> &'a T {
        let uuid = S::uuid();
        let id = std::any::TypeId::of::<T>();

        let widget = std::any::type_name::<S>();
        let type_nm = std::any::type_name::<T>();

        self
            ._inner
            .get(&uuid)
            .expect(&format!("There is no resource allocated for {widget}"))
            .get(&id)
            .expect(&format!("There is no resource {type_nm} for {widget}"))
            .downcast_ref::<T>()
            .expect("Error while downcasting to &{type_nm}")
    }

    pub fn get_mut<'a, S: GetUuid, T: 'static>(&'a mut self) -> &'a mut T {
        let uuid = S::uuid();
        let id = std::any::TypeId::of::<T>();

        let widget = std::any::type_name::<S>();
        let type_nm = std::any::type_name::<T>();

        self
            ._inner
            .get_mut(&uuid)
            .expect(&format!("There is no resource allocated for {widget}"))
            .get_mut(&id)
            .expect(&format!("There is no resource {type_nm} for {widget}"))
            .downcast_mut::<T>()
            .expect("Error while downcasting to &{type_nm}")
    }
}
