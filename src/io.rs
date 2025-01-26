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

use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Deref, DerefMut};
use std::path::Path;

use anyhow::anyhow;
use ratatui::layout::*;

/// A list to maintain names of the file. The actual file content will be saved
/// into another object this type only to provide an ordered list of file names.
#[derive(Default)]
pub struct FileList {
    pub table: Vec<FileBuf>,
}

impl Deref for FileList {
    type Target = Vec<FileBuf>;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl DerefMut for FileList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

impl FileList {
    pub fn with_files(files: Vec<String>) -> anyhow::Result<Self> {
        let mut table = Vec::new();
        for file in files.into_iter() {
            table.push(FileBuf::new(&file, false)?);
        }
        Ok(FileList { table })
    }

    pub fn insert(&mut self, name: &str) -> anyhow::Result<()> {
        let file = FileBuf::new(name, false)?;

        if self.table.iter().any(|f| f.path() == file.path()) {
            return Err(anyhow!(
                "File with path {} is already open",
                file.path.display()
            ));
        }

        self.table.push(file);
        Ok(())
    }

    pub fn close(&mut self, id: usize) {
        self.table.remove(id);
    }

    #[inline]
    pub fn names(&self) -> Vec<&str> {
        self.iter().map(|f| f.name()).collect()
    }

    #[inline]
    pub fn get_file_buff(&self, index: usize) -> Option<&FileBuf> {
        self.get(index)
    }

    #[inline]
    pub fn get_file_buff_mut(&mut self, index: usize) -> Option<&mut FileBuf> {
        self.get_mut(index)
    }
}

pub struct FileBuf {
    nulled: bool,
    is_tail: bool,
    name: Box<str>,
    path: Box<Path>,
    reader: Option<BufReader<File>>,
    view: RefCell<[usize; 2]>,
    view_update: bool,
    lines: usize,
    buffer: Vec<String>,
    max_scroll_limit: u16, // horizontal scroll
    current_scroll: u16,
}

impl FileBuf {
    pub fn new(path: &str, is_tail: bool) -> anyhow::Result<Self> {
        let nulled = false;

        let name = path.to_string().into_boxed_str();
        let path = Path::new(path).canonicalize()?.into_boxed_path();
        let file = File::open(path.as_ref())?;
        let reader = Some(BufReader::new(file));
        let buffer = Vec::new();
        let view = RefCell::new(Default::default());
        let view_update = true;
        let lines = 1;
        let max_scroll_limit = 0;
        let current_scroll = 0;

        log::trace!("Opening a file with path {}", path.display());

        Ok(FileBuf {
            nulled,
            is_tail,
            name,
            path,
            reader,
            view,
            lines,
            view_update,
            buffer,
            max_scroll_limit,
            current_scroll,
        })
    }

    pub fn update(&mut self) -> anyhow::Result<()> {
        if self.is_tail && self.reader.is_none() {
            let file = File::open(self.path())?;
            self.reader = Some(BufReader::new(file));
        }

        let Some(reader) = self.reader.as_mut() else {
            return Ok(());
        };

        let mut lines_to_read = self.view.borrow()[1];
        let mut buffer = String::new();

        while lines_to_read > 0 {
            if reader.read_line(&mut buffer)? == 0 && !self.is_tail {
                self.reader = None;
                break;
            }

            let line = format!(
                "{:>6}|  {}",
                self.lines,
                buffer
                    .replace('\t', &"\u{000A0}".repeat(4))
                    .replace('\r', "")
            );

            self.buffer.push(line);

            self.lines += 1;
            lines_to_read -= 1;
            buffer.clear();
        }

        Ok(())
    }

    pub fn detrigger_view_update(&mut self) {
        if self.view_update {
            self.view_update = false;
        }
    }

    pub fn trigger_view_update(&mut self) {
        if self.view_update {
            return;
        }
        self.view_update = true;
    }

    // Only return lines that are visible on the screen.
    pub fn buffer<'a>(&'a self, rect: Rect) -> (Vec<&'a str>, bool) {
        if self.view_update {
            let mut view = self.view.borrow_mut();
            view[1] = view[0]
                .saturating_add(rect.as_size().height.into())
                .saturating_sub(2);
        }

        let len = self.buffer.len();
        let (start, end) = (self.view.borrow()[0], self.view.borrow()[1]);
        let lines = self
            .buffer
            .iter()
            .map(String::as_str)
            .collect::<Vec<&'a str>>();

        let slice = if len >= end && start < end {
            &lines[start..end]
        } else {
            &lines[..]
        };

        (slice.to_vec(), self.nulled)
    }

    // Replace the buffer with the error message and close the file reader.
    pub fn nullify(&mut self, message: String) {
        self.nulled = true;
        self.buffer = vec![message];
        let _ = self.reader.take();
        self.view = RefCell::new([0, 1]);
        self.is_tail = false;
    }

    pub fn next(&mut self) {
        let len = self.buffer.len();
        let (start, end) = (self.view.borrow()[0], self.view.borrow()[1]);
        if end < len {
            let mut view = self.view.borrow_mut();
            view[0] = start.saturating_add(1);
            view[1] = end.saturating_add(1);
        }
    }

    pub fn prev(&mut self) {
        let (start, end) = (self.view.borrow()[0], self.view.borrow()[1]);
        if start > 0 {
            let mut view = self.view.borrow_mut();
            view[0] = start.saturating_sub(1);
            view[1] = end.saturating_sub(1);
        }
    }

    pub fn top(&mut self) {
        let (start, end) = (self.view.borrow()[0], self.view.borrow()[1]);
        if start > 0 {
            let mut view = self.view.borrow_mut();
            view[0] = 0;
            view[1] = end - start;
        }
    }

    pub fn bottom(&mut self) {
        let len = self.buffer.len();
        let (start, end) = (self.view.borrow()[0], self.view.borrow()[1]);
        if end < len {
            let diff = len.saturating_sub(end);
            let mut view = self.view.borrow_mut();
            view[0] = start.saturating_add(diff);
            view[1] = end.saturating_add(diff);
        }
    }

    pub fn set_max_scroll_limit(&mut self, l: u16) {
        self.max_scroll_limit = l;
    }

    pub fn scroll_next(&mut self) {
        self.current_scroll = self.current_scroll.saturating_add(1).min(self.max_scroll_limit);
    }

    pub fn scroll_prev(&mut self) {
        self.current_scroll = self.current_scroll.saturating_sub(1);
    }

    pub fn get_current_scroll(&self) -> (u16, u16) {
        (self.current_scroll, 0)
    }

    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
}
