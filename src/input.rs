/*
 * MIT License
 *
 * Copyright (c) 2024 Mohammed Rehaan and contributors
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * */

use std::fs::read_dir;

use anyhow::anyhow;
use crossterm::event::*;
use ratatui::widgets::*;

use crate::cursor::*;
use crate::resource::*;
use crate::*;
use io::FileBuf;
use utils::complete;

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

    pub fn complete(&mut self) {
        let mut new_buff = if self.input_buff.starts_with("/") {
            self.input_buff.to_string()
        } else {
            "./".to_string() + &self.input_buff
        };

        let path: Vec<&str> = new_buff.split("/").collect();
        let path = path[..path.len().saturating_sub(1)].join("/") + "/";

        let Ok(filenames) = read_dir(path) else {
            return;
        };
        let filenames = filenames
            .filter_map(|file| file.ok())
            .map(|file| {
                let is_dir = if let Ok(md) = file.metadata() {
                    md.is_dir()
                } else {
                    false
                };

                file.path().display().to_string() + if is_dir { "/" } else { "" }
            })
            .collect();

        new_buff = complete(filenames, &new_buff);
        new_buff = match new_buff.strip_prefix("./") {
            Some(s) => s.to_string(),
            None => new_buff,
        };

        self.input_buff = new_buff;
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

    pub fn close(&mut self) -> anyhow::Result<usize> {
        if self.size == 0 {
            return Err(anyhow!("ListState empty"));
        }
        let res = self.index;
        self.size = self.size.saturating_sub(1);
        self.index = self.index.min(self.size.saturating_sub(1));
        self.state.select(Some(self.index));
        Ok(res)
    }
}

pub struct KeyListenerResponse {
    should_exit: bool,
    polled: bool,
}

impl KeyListenerResponse {
    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    pub fn polled(&self) -> bool {
        self.polled
    }
}

pub fn key_listener(res: &mut Resource) -> anyhow::Result<KeyListenerResponse> {
    let mut polled = false;

    if poll(std::time::Duration::from_millis(DELTA))? {
        polled = true;

        let event = read()?;
        if quit_listener(&event) {
            return Ok(KeyListenerResponse {
                should_exit: true,
                polled,
            });
        }
        match res.entry_box().is_visible() {
            true => write_key_event(event, res)?,
            false => normal_key_event(event, res),
        }
    }


    Ok(KeyListenerResponse {
        should_exit: false,
        polled,
    })
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
            code: KeyCode::Char('d'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => {
            let id = match res.file_list_state_mut().close() {
                Ok(id) => id,
                Err(_) => return,
            };
            res.files_mut().close(id);
        }

        Event::Key(KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => res.pointer_mut().set_cursor::<Files>(),

        Event::Key(KeyEvent {
            code: KeyCode::Char('l'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => res.pointer_mut().set_cursor::<View>(),

        Event::Key(KeyEvent {
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
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => {
            if res.pointer().cursor_at::<Files>() {
                res.file_list_state_mut().bottom();
            }
            if res.pointer().cursor_at::<View>() {
                let curr_index = res.file_list_state().index();
                if let Some(curr_buff) = res.files_mut().get_file_buff_mut(curr_index) {
                    curr_buff.bottom();
                }
            }
        }

        Event::Key(KeyEvent {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => {
            if res.pointer().cursor_at::<Files>() {
                res.file_list_state_mut().top();
            }
            if res.pointer().cursor_at::<View>() {
                let curr_index = res.file_list_state().index();
                if let Some(curr_buff) = res.files_mut().get_file_buff_mut(curr_index) {
                    curr_buff.top();
                }
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
            code: KeyCode::Tab, ..
        }) => {
            res.entry_box.complete();
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
