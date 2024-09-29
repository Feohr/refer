use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Deref, DerefMut};
use std::cell::RefCell;

use ratatui::layout::*;

/// A list to maintain names of the file. The actual file content will be saved
/// into another object this type only to provide an ordered list of file names.
#[derive(Default)]
pub struct FileList {
    pub table: Vec<(FileName, FileBuf)>,
}

impl Deref for FileList {
    type Target = Vec<(FileName, FileBuf)>;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl DerefMut for FileList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

#[derive(Default, Hash)]
pub struct FileName {
    value: Box<str>,
}

impl FileName {
    #[inline]
    fn new(value: Box<str>) -> Self {
        FileName { value }
    }

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl FileList {
    pub fn with_files(files: Vec<String>) -> anyhow::Result<Self> {
        let mut table = Vec::new();
        for file in files.into_iter() {
            table.push((
                FileName::new(file.clone().into()),
                FileBuf::new(file, false)?,
            ));
        }
        Ok(FileList { table })
    }

    pub fn insert(&mut self, name: Box<str>) -> anyhow::Result<()> {
        self.table.push((
            FileName::new(name.clone()),
            FileBuf::new(name.to_string(), false)?,
        ));
        Ok(())
    }

    #[inline]
    pub fn names(&self) -> Vec<&FileName> {
        self.iter().map(|(f, _)| f).collect()
    }

    #[inline]
    pub fn get_file_buff(&self, index: usize) -> Option<&FileBuf> {
        self.get(index).map(|(_, f)| f)
    }

    #[inline]
    pub fn get_file_buff_mut(&mut self, index: usize) -> Option<&mut FileBuf> {
        self.get_mut(index).map(|(_, f)| f)
    }
}

pub struct FileBuf {
    nulled: bool,
    is_tail: bool,
    path: Box<str>,
    reader: Option<BufReader<File>>,
    view: RefCell<[usize; 2]>,
    view_update: bool,
    lines: usize,
    buffer: Vec<String>,
}

impl FileBuf {
    pub fn new(path: String, is_tail: bool) -> anyhow::Result<Self> {
        let nulled = false;
        let path = path.to_string().into_boxed_str();
        let file = File::open(path.as_ref())?;
        let reader = Some(BufReader::new(file));
        let buffer = Vec::new();
        let view = RefCell::new(Default::default());
        let view_update = true;
        let lines = 1;

        Ok(FileBuf {
            nulled,
            is_tail,
            path,
            reader,
            view,
            lines,
            view_update,
            buffer,
        })
    }

    pub fn update(&mut self) -> anyhow::Result<()> {
        if self.is_tail && self.reader.is_none() {
            let file = File::open(self.path.as_ref())?;
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

            self.buffer.push(format!(
                "{:>6}|  {}",
                self.lines,
                buffer
                    .replace('\t', &"\u{000A0}".repeat(4))
                    .replace('\r', "")
            ));

            self.lines += 1;
            lines_to_read -= 1;
            buffer.clear();
        }

        Ok(())
    }

    pub fn detrigger_view_update(&mut self) {
        if self.view_update {
            log::debug!("Detriggering view update for: {}", self.path);
            self.view_update = false;
        }
    }

    pub fn trigger_view_update(&mut self) {
        if self.view_update { return }
        log::debug!("Triggering view update for: {}", self.path);
        self.view_update = true;
    }

    // Only return lines that are visible on the screen.
    pub fn buffer<'a>(&'a self, rect: Rect) -> (Vec<&'a str>, bool) {
        if self.view_update {
            let mut view = self.view.borrow_mut();
            view[1] = view[0].saturating_add(rect.as_size().height.into()).saturating_sub(2);
        }

        let len = self.buffer.len();
        let (start, end) = (self.view.borrow()[0], self.view.borrow()[1]);
        log::debug!("In path: {}, start: {} and end: {}", self.path, start, end);
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
            log::debug!("inside next. start: {}, end: {}", start, end);
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
}
