use std::cell::{Ref, RefCell, RefMut};

use clap::Parser;

use crate::cursor::*;
use crate::input::*;
use crate::io::*;

pub struct Resource {
    pub pointer: KeyboardCursor,
    pub entry_box: EntryBox,
    pub file_list_state: RefCell<FileListState>,
    pub files: FileList,
}

impl Resource {
    pub fn new() -> anyhow::Result<Self> {
        let args = Refer::parse();
        let files = FileList::with_files(args.filename)?;

        Ok(Resource {
            pointer: KeyboardCursor::new(),
            entry_box: EntryBox::new(),
            file_list_state: RefCell::new(FileListState::new(files.len())),
            files,
        })
    }

    #[inline]
    pub fn pointer(&self) -> &KeyboardCursor {
        &self.pointer
    }

    #[inline]
    pub fn entry_box(&self) -> &EntryBox {
        &self.entry_box
    }

    #[inline]
    pub fn file_list_state(&self) -> Ref<FileListState> {
        self.file_list_state.borrow()
    }

    #[inline]
    pub fn files(&self) -> &FileList {
        &self.files
    }

    #[inline]
    pub fn pointer_mut(&mut self) -> &mut KeyboardCursor {
        &mut self.pointer
    }

    #[inline]
    pub fn entry_box_mut(&mut self) -> &mut EntryBox {
        &mut self.entry_box
    }

    #[inline]
    pub fn file_list_state_mut(&self) -> RefMut<FileListState> {
        self.file_list_state.borrow_mut()
    }

    #[inline]
    pub fn files_mut(&mut self) -> &mut FileList {
        &mut self.files
    }
}

#[derive(Parser)]
#[command(about, long_about=None)]
struct Refer {
    filename: Vec<String>,
}

pub fn state_update(res: &mut Resource) {
    res.files_mut().iter_mut().for_each(|(_, f)| {
        if let Err(err) = f.update() {
            f.nullify(format!("{err}"));
        }
    });
}
