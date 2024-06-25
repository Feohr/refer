use clap::Parser;

use crate::cursor::*;
use crate::input::*;

pub struct Resource {
    pub pointer: Pointer,
    pub entry_box: EntryBox,
    pub file_list_state: FileListState,
    pub file_buff: FileBuff,
}

impl Resource {
    pub fn new() -> Self {
        let args = Refer::parse();
        let file_buff = FileBuff::with_files(args.filename);

        Resource {
            pointer: Pointer::new(),
            entry_box: EntryBox::new(),
            file_list_state: FileListState::new(file_buff.len()),
            file_buff,
        }
    }

    pub fn pointer(&self) -> &Pointer {
        &self.pointer
    }

    pub fn entry_box(&self) -> &EntryBox {
        &self.entry_box
    }

    pub fn file_list_state(&self) -> &FileListState {
        &self.file_list_state
    }

    pub fn file_buff(&self) -> &FileBuff {
        &self.file_buff
    }

    pub fn pointer_mut(&mut self) -> &mut Pointer {
        &mut self.pointer
    }

    pub fn entry_box_mut(&mut self) -> &mut EntryBox {
        &mut self.entry_box
    }

    pub fn file_list_state_mut(&mut self) -> &mut FileListState {
        &mut self.file_list_state
    }

    pub fn file_buff_mut(&mut self) -> &mut FileBuff {
        &mut self.file_buff
    }
}

#[derive(Parser)]
#[command(about, long_about=None)]
struct Refer {
    filename: Vec<String>,
}
