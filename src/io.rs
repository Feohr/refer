use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Deref, DerefMut};

use ratatui::layout::*;

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
    fn new(value: Box<str>) -> Self {
        FileName { value }
    }

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

    pub fn names(&self) -> Vec<&FileName> {
        self.iter().map(|(f, _)| f).collect()
    }

    pub fn get_file_buff(&self, index: usize) -> Option<&FileBuf> {
        self.get(index).map(|(_, f)| f)
    }
}

pub struct FileBuf {
    is_tail: bool,
    path: Box<str>,
    reader: Option<BufReader<File>>,
    count: usize,
    buffer: String,
}

impl FileBuf {
    pub fn new(path: String, is_tail: bool) -> anyhow::Result<Self> {
        let path = path.to_string().into_boxed_str();
        let file = File::open(path.as_ref())?;
        let reader = Some(BufReader::new(file));
        let buffer = String::new();
        let count = 1;

        Ok(FileBuf {
            is_tail,
            path,
            reader,
            count,
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

                self.buffer.push_str(&format!(
                    "{:>6}|  {}",
                    self.count,
                    buffer.replace('\t', "\\t")
                ));

                self.count += 1;
                lines_to_read -= 1;
                buffer.clear();
            }
        }

        Ok(())
    }

    // Only return lines that are visible in the screen.
    pub fn buffer<'a>(&'a self, rect: Rect) -> Vec<&'a str> {
        let size = rect.height.saturating_sub(rect.y) as usize;
        let lines = self.buffer.lines().collect::<Vec<&'a str>>();
        let min = lines.len().min(size);
        lines[0..min].to_vec()
    }
}
