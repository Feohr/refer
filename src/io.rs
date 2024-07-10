use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Deref, DerefMut};

use ratatui::layout::*;

use crate::*;

/// A list to maintain names of the file. The actual file content will be saved into another object
/// this type only to provide an ordered list of file names.
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
    index: usize,
    count: usize,
    buffer: Vec<String>,
}

impl FileBuf {
    pub fn new(path: String, is_tail: bool) -> anyhow::Result<Self> {
        let nulled = false;
        let path = path.to_string().into_boxed_str();
        let file = File::open(path.as_ref())?;
        let reader = Some(BufReader::new(file));
        let buffer = Vec::new();
        let count = 1;
        let index = 0;

        Ok(FileBuf {
            nulled,
            is_tail,
            path,
            reader,
            count,
            index,
            buffer,
        })
    }

    pub fn update(&mut self) -> anyhow::Result<()> {
        if self.is_tail && self.reader.is_none() {
            let file = File::open(self.path.as_ref())?;
            self.reader = Some(BufReader::new(file));
        }

        if let Some(reader) = self.reader.as_mut() {
            let mut lines_to_read = 48; // Read 48 lines at a time.
            let mut buffer = String::new();

            while lines_to_read > 0 {
                if reader.read_line(&mut buffer)? == 0 && !self.is_tail {
                    self.reader = None;
                    break;
                }

                self.buffer.push(format!(
                    "{:>6}|  {}",
                    self.count,
                    buffer.replace('\t', &"\u{000A0}".repeat(4)).replace('\r', "")
                ));

                self.count += 1;
                lines_to_read -= 1;
                buffer.clear();
            }
        }

        Ok(())
    }

    // Only return lines that are visible in the screen.
    pub fn buffer<'a>(&'a self, rect: Rect) -> (Vec<&'a str>, bool) {
        let size = rect.height.saturating_sub(rect.y) as usize;
        let lines = self
            .buffer
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&'a str>>();

        let len = lines.len();
        let bottom = self.index + size;
        let min = bottom.min(len);

        (lines[self.index..min].to_vec(), self.nulled)
    }

    // Replace the buffer with the error message and close the file reader.
    pub fn nullify(&mut self, message: String) {
        self.nulled = true;
        self.buffer = vec![message];
        self.reader = None;
        self.index = 0;
        self.is_tail = false;
    }

    pub fn next(&mut self) {
        if self.nulled {
            return;
        }
        self.index = bounded_add(self.index, 1, self.count.saturating_sub(2));
    }

    pub fn prev(&mut self) {
        if self.nulled {
            return;
        }
        self.index = self.index.saturating_sub(1);
    }
}
