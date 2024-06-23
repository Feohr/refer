use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};
use std::collections::{hash_map::Keys, HashMap};
use std::ops::Add;
use std::ops::Deref;

use crossterm::event::*;
use ratatui::widgets::*;

use crate::cursor::*;
use crate::resource::*;

pub const DELTA: u64 = 16;

trait MaxedAdd<Rhs = Self> {
    type Output;
    fn max_add(self, other: Rhs, max: Rhs) -> Self::Output;
}

impl<T: Add<Output = T> + PartialOrd + Sized> MaxedAdd<T> for T {
    type Output = T;
    fn max_add(self, other: T, max: T) -> Self::Output {
        if self < max {
            return self.add(other);
        }
        self
    }
}

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

    pub fn get_buffer(&self, index: usize) -> &String {
        self.table
            .iter()
            .find(|(f, _)| f.index.eq(&index))
            .expect("Buffer not present for the file {name}")
            .1
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

pub struct FileListState {
    size: usize,
    pub index: usize,
    pub state: ListState,
}
impl FileListState {
    pub fn new(size: usize) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        FileListState {
            size,
            index: 0,
            state,
        }
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn next(&mut self) {
        self.index = self.index.max_add(1, self.size.saturating_sub(1));
        self.state.select(Some(self.index));
    }

    pub fn prev(&mut self) {
        self.index = self.index.saturating_sub(1);
        self.state.select(Some(self.index));
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn get_mut(&mut self) -> &mut ListState {
        &mut self.state
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
        }) => res.get_mut::<Pointer>().set_cursor::<Files>(),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            ..
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Char('l'),
            ..
        }) => res.get_mut::<Pointer>().set_cursor::<View>(),
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            ..
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Char('j'),
            ..
        }) => {
            if res.get::<Pointer>().cursor_at::<Files>() {
                res.get_mut::<FileListState>().next();
            }
        }
        Event::Key(KeyEvent {
            code: KeyCode::Up, ..
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Char('k'),
            ..
        }) => {
            if res.get::<Pointer>().cursor_at::<Files>() {
                res.get_mut::<FileListState>().prev();
            }
        }
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

            let len = res.get::<FileBuff>().len();
            res.get_mut::<FileListState>().set_size(len);

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
