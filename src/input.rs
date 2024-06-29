use std::cmp::PartialOrd;
use std::ops::Add;

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

    pub fn is_visible(&self) -> bool {
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

    pub fn bottom(&mut self) {
        self.index = self.size.saturating_sub(1);
        self.state.select(Some(self.index));
    }

    pub fn prev(&mut self) {
        self.index = self.index.saturating_sub(1);
        self.state.select(Some(self.index));
    }

    pub fn top(&mut self) {
        self.index = 0;
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
        match res.entry_box().is_visible() {
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
            modifiers: KeyModifiers::CONTROL,
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
            res.pointer_mut().toggle();
            res.entry_box_mut().toggle();
        }
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            ..
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Char('h'),
            ..
        }) => res.pointer_mut().set_cursor::<Files>(),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            ..
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Char('l'),
            ..
        }) => res.pointer_mut().set_cursor::<View>(),
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            ..
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::NONE,
            ..
        }) => {
            if res.pointer().cursor_at::<Files>() {
                res.file_list_state_mut().next();
            }
        }
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
            ..
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::NONE,
            ..
        }) => {
            if res.pointer().cursor_at::<Files>() {
                res.file_list_state_mut().prev();
            }
        }
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::CONTROL,
            ..
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => {
            if res.pointer().cursor_at::<Files>() {
                res.file_list_state_mut().bottom();
            }
        }
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::CONTROL,
            ..
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => {
            if res.pointer().cursor_at::<Files>() {
                res.file_list_state_mut().top();
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
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
            ..
        })
        => {
            res.pointer_mut().toggle();
            res.entry_box_mut().clear();
            res.entry_box_mut().toggle();
        }
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            ..
        }) => {
            let name = res.entry_box_mut().take();

            if !name.is_empty() {
                res.files_mut().insert(name);
                let len = res.files().len();
                res.file_list_state_mut().set_size(len);
            }

            res.pointer_mut().toggle();
            res.entry_box_mut().toggle();
        }
        Event::Key(KeyEvent {
            code: KeyCode::Backspace,
            ..
        }) => res.entry_box_mut().pop(),
        Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        }) => res.entry_box_mut().push(c),
        _ => {}
    }
}
