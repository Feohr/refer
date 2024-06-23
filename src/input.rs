use std::collections::{HashMap, hash_map::Keys};
use std::ops::Deref;

use crossterm::event::*;

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
    table: HashMap<String, String>,
}

impl FileBuff {
    pub fn names(&self) -> Keys<'_, String, String> {
        self.table.keys()
    }

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
