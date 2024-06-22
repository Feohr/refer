use std::collections::HashMap;
use std::ops::Deref;

pub struct EntryBox {
    is_active: bool,
    input_buff: String,
}

impl EntryBox {
    pub fn new() -> Self {
        EntryBox {
            is_active: false,
            input_buff: String::new(),
        }
    }

    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    pub fn bool(&self) -> bool {
        self.is_active
    }

    pub fn push(&mut self, ch: char) {
        self.input_buff.push(ch)
    }

    pub fn pop(&mut self) {
        self.input_buff.pop();
    }

    pub fn len(&self) -> usize {
        self.input_buff.len()
    }

    pub fn take(&mut self) -> String {
        std::mem::take(&mut self.input_buff)
    }

    pub fn get(&self) -> &str {
        &self.input_buff
    }

    pub fn get_span(&self, width: usize) -> &str {
        let len = self.input_buff.len();
        let offset = len.saturating_sub(width);
        &self.input_buff[offset..len]
    }
}

#[derive(Default)]
pub struct FileBuff {
    table: HashMap<String, String>,
}

impl FileBuff {
    pub fn get(&self, name: &String) -> &String {
        self.table
            .get(name)
            .expect("Buffer not present for the file {name}")
    }

    pub fn insert(&mut self, name: String) {
        self.table.insert(name, String::new());
    }
}

impl Deref for FileBuff {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}
