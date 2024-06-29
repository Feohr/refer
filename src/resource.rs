use std::cell::{RefCell, Ref, RefMut};

use clap::Parser;

use crate::io::*;
use crate::cursor::*;
use crate::input::*;

pub struct Resource {
    pub pointer: KeyboardCursor,
    pub entry_box: EntryBox,
    pub file_list_state: RefCell<FileListState>,
    pub files: FileList,
}

impl Resource {
    pub fn new() -> Self {
        let args = Refer::parse();
        let files = FileList::with_files(args.filename);

        Resource {
            pointer: KeyboardCursor::new(),
            entry_box: EntryBox::new(),
            file_list_state: RefCell::new(FileListState::new(files.len())),
            files,
        }
    }

    pub fn pointer(&self) -> &KeyboardCursor {
        &self.pointer
    }

    pub fn entry_box(&self) -> &EntryBox {
        &self.entry_box
    }

    pub fn file_list_state(&self) -> Ref<FileListState> {
        self.file_list_state.borrow()
    }

    pub fn files(&self) -> &FileList {
        &self.files
    }

    pub fn pointer_mut(&mut self) -> &mut KeyboardCursor {
        &mut self.pointer
    }

    pub fn entry_box_mut(&mut self) -> &mut EntryBox {
        &mut self.entry_box
    }

    pub fn file_list_state_mut(&self) -> RefMut<FileListState> {
        self.file_list_state.borrow_mut()
    }

    pub fn files_mut(&mut self) -> &mut FileList {
        &mut self.files
    }
}

#[derive(Parser)]
#[command(about, long_about=None)]
struct Refer {
    filename: Vec<String>,
}
