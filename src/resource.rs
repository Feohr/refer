use std::any::{Any, TypeId};
use std::collections::HashMap;

use clap::Parser;

use crate::input::*;
use crate::cursor::*;

#[derive(Default)]
pub struct Resource {
    _inner: HashMap<TypeId, Box<dyn Any>>,
}

impl Resource {
    pub fn insert<T: 'static>(&mut self, item: T) {
        let id = std::any::TypeId::of::<T>();
        if self._inner.get(&id).is_some() {
            panic!(
                "Resource of type {} is already present",
                std::any::type_name::<T>()
            );
        }
        self._inner.insert(id, Box::new(item));
    }

    pub fn get<'a, T: 'static>(&'a self) -> &'a T {
        let id = std::any::TypeId::of::<T>();

        let type_nm = std::any::type_name::<T>();

        self._inner
            .get(&id)
            .expect(&format!("The resource {type_nm} was never allocated"))
            .downcast_ref::<T>()
            .expect("Error while downcasting to &{type_nm}")
    }

    pub fn get_mut<'a, T: 'static>(&'a mut self) -> &'a mut T {
        let id = std::any::TypeId::of::<T>();

        let type_nm = std::any::type_name::<T>();

        self._inner
            .get_mut(&id)
            .expect(&format!("The resource {type_nm} was never allocated"))
            .downcast_mut::<T>()
            .expect("Error while downcasting to &{type_nm}")
    }
}

#[derive(Parser)]
#[command(about, long_about=None)]
struct Refer {
    filename: Vec<String>,
}

pub fn init_resource() -> anyhow::Result<Resource> {
    let args = Refer::parse();

    let mut resource = Resource::default();
    resource.insert(Pointer::new());
    resource.insert(EntryBox::new());
    resource.insert(FileBuff::with_files(args.filename));
    resource.insert(FileListState::new());

    Ok(resource)
}
