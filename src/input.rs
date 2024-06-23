use std::cmp::{Eq, PartialEq, PartialOrd, Ordering};
use std::collections::{HashMap, hash_map::Keys};
use std::ops::Deref;

use crossterm::event::*;
use ratatui::widgets::*;

use crate::resource::*;
use crate::cursor::*;

pub const DELTA: u64 = 16;

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
        self.input_buff.push(ch);
    }

    pub fn pop(&mut self) {
        self.input_buff.pop();
    }

    pub fn len(&self) -> usize {
        self.input_buff.len()
    }

    pub fn clear(&mut self) {
        self.input_buff.clear();
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
    table: HashMap<FileName, String>,
}

#[derive(Default, Clone, Eq, Ord, Hash)]
pub struct FileName {
    value: String,
    index: usize,
}

impl PartialEq for FileName {
    fn eq(&self, other: &Self) -> bool {
        self.value().eq(other.value())
    }
}

impl PartialOrd for FileName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl FileName {
    fn new(value: String, index: usize) -> Self {
        FileName { value, index }
    }

    fn value(&self) -> &str {
        &self.value
    }

    pub fn to_value(&self) -> String {
        self.value.clone()
    }
}

impl FileBuff {
    pub fn with_files(files: Vec<String>) -> Self {
        let mut table = HashMap::new();
        for (index, file) in files.into_iter().enumerate() {
            table.insert(FileName::new(file, index), String::new());
        }
        FileBuff { table }
    }

    pub fn names(&self) -> Keys<'_, FileName, String> {
        self.table.keys()
    }

    pub fn get_buffer(&self, value: &String) -> &String {
        self.table
            .get(&FileName {
                value: value.to_string(),
                ..Default::default()
            })
            .expect("Buffer not present for the file {name}")
    }

    pub fn insert(&mut self, name: String) {
        let len = self.table.len();
        self.table.insert(FileName::new(name, len), String::new());
    }
}

impl Deref for FileBuff {
    type Target = HashMap<FileName, String>;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

pub struct FileListState(pub ListState);
impl FileListState {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        FileListState(state)
    }

    pub fn get_mut(&mut self) -> &mut ListState {
        &mut self.0
    }
}

pub fn key_listener(res: &mut Resource) -> anyhow::Result<bool> {
    if poll(std::time::Duration::from_millis(DELTA))? {
        let event = read()?;
        if quit_listener(&event) {
            return Ok(true);
        }
        match res.get::<EntryBox>().bool() {
            true => write_key_event(event, res),
            false => normal_key_event(event, res),
        }
    }

    Ok(false)
}

fn quit_listener(event: &Event) -> bool {
    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            ..
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
            ..
        }) => return true,
        _ => {}
    }
    false
}

fn normal_key_event(event: Event, res: &mut Resource) {
    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char('n'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => {
            res.get_mut::<Pointer>().toggle();
            res.get_mut::<EntryBox>().toggle();
        }
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            ..
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Char('h'),
            ..
        })
        => res.get_mut::<Pointer>().set_cursor::<Files>(),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            ..
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Char('l'),
            ..
        })
        => res.get_mut::<Pointer>().set_cursor::<View>(),
        _ => {}
    }
}

fn write_key_event(event: Event, res: &mut Resource) {
    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char('n'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => {
            res.get_mut::<Pointer>().toggle();
            res.get_mut::<EntryBox>().clear();
            res.get_mut::<EntryBox>().toggle();
        }
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            ..
        }) => {
            let name = res.get_mut::<EntryBox>().take();
            res.get_mut::<FileBuff>().insert(name);
            res.get_mut::<Pointer>().toggle();
            res.get_mut::<EntryBox>().toggle();
        }
        Event::Key(KeyEvent {
            code: KeyCode::Backspace,
            ..
        }) => res.get_mut::<EntryBox>().pop(),
        Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        }) => res.get_mut::<EntryBox>().push(c),
        _ => {}
    }
}
