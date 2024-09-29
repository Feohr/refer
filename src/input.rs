use crossterm::event::*;
use io::FileBuf;
use ratatui::widgets::*;

use crate::cursor::*;
use crate::resource::*;
use crate::*;

pub const DELTA: u64 = 16;

#[derive(Default)]
pub struct EntryBox {
    is_active: bool,
    is_err: bool,
    input_buff: String,
}

impl EntryBox {
    #[inline]
    pub fn new() -> Self {
        EntryBox::default()
    }

    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    #[inline]
    pub fn is_visible(&self) -> bool {
        self.is_active
    }

    pub fn push(&mut self, ch: char) {
        self.input_buff.push(ch);
    }

    pub fn pop(&mut self) {
        self.input_buff.pop();
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.input_buff.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.input_buff.len() == 0
    }

    pub fn clear(&mut self) {
        self.input_buff.clear();
    }

    #[inline]
    pub fn take(&mut self) -> String {
        std::mem::take(&mut self.input_buff)
    }

    #[inline]
    pub fn get(&self) -> &str {
        &self.input_buff
    }

    pub fn set_err(&mut self) {
        self.is_err = true;
    }

    pub fn set_ok(&mut self) {
        self.is_err = false;
    }

    #[inline]
    pub fn is_err(&self) -> bool {
        self.is_err
    }

    #[inline]
    pub fn input_buff(&self) -> Box<str> {
        self.input_buff.clone().into_boxed_str()
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
        self.index = bounded_add(self.index, 1, self.size.saturating_sub(1));
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

    #[inline]
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
            true => write_key_event(event, res)?,
            false => normal_key_event(event, res),
        }
    }

    Ok(false)
}

pub fn trigger_view_update(res: &mut Resource) {
    res.files_mut()
        .iter_mut()
        .for_each(FileBuf::trigger_view_update);
}

pub fn detrigger_view_update(res: &mut Resource) {
    res.files_mut()
        .iter_mut()
        .for_each(FileBuf::detrigger_view_update);
}

fn quit_listener(event: &Event) -> bool {
    if let Event::Key(KeyEvent {
        code: KeyCode::Char('q'),
        modifiers: KeyModifiers::CONTROL,
        ..
    }) = event
    {
        return true;
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
            if res.pointer().cursor_at::<View>() {
                let curr_index = res.file_list_state().index();
                if let Some(curr_buff) = res.files_mut().get_file_buff_mut(curr_index) {
                    curr_buff.next();
                }
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
            if res.pointer().cursor_at::<View>() {
                let curr_index = res.file_list_state().index();
                if let Some(curr_buff) = res.files_mut().get_file_buff_mut(curr_index) {
                    curr_buff.prev();
                }
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

fn write_key_event(event: Event, res: &mut Resource) -> anyhow::Result<()> {
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
        }) => {
            res.pointer_mut().toggle();
            res.entry_box_mut().clear();
            res.entry_box_mut().toggle();
        }
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            ..
        }) => {
            let name = res.entry_box().input_buff();
            if let Err(err) = res.files_mut().insert(&name) {
                log::trace!("Cannot open file due to: {err}");
                res.entry_box_mut().set_err();
                return Ok(());
            }

            // Clean the entry box
            res.entry_box_mut().clear();

            if name.is_empty() {
                return Ok(());
            }
            let len = res.files().len();
            res.file_list_state_mut().set_size(len);
            res.entry_box_mut().set_ok();
            res.pointer_mut().toggle();
            res.entry_box_mut().toggle();
        }
        Event::Key(KeyEvent {
            code: KeyCode::Backspace,
            ..
        }) => {
            res.entry_box_mut().set_ok();
            res.entry_box_mut().pop();
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        }) => res.entry_box_mut().push(c),
        _ => {}
    }

    Ok(())
}
