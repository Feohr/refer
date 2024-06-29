use std::ops::Deref;
use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};

/// A list to maintain names of the file. The actual file content will be saved into another object
/// this type only to provide an ordered list of file names.
#[derive(Default)]
pub struct FileList {
    pub table: Vec<FileName>,
}

impl Deref for FileList {
    type Target = Vec<FileName>;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

#[derive(Default, Clone, Eq, Ord, Hash)]
pub struct FileName {
    value: Box<str>,
    /// Technically, this wouldn't be required since vector is a stack and the files that are
    /// opened will be saved in sorted anyways. Still, this is used for extra guarantee that the
    /// file names aren't jumbled and the appropriate file buffers are accessed.
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
        FileName { value: value.into(), index }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl FileList {
    pub fn with_files(files: Vec<String>) -> Self {
        let mut table = Vec::new();
        for (index, file) in files.into_iter().enumerate() {
            table.push(FileName::new(file, index));
        }
        FileList { table }
    }

    pub fn insert(&mut self, name: String) {
        let len = self.table.len();
        self.table.push(FileName::new(name, len));
    }
}

// /// Window for file content. The variables `start` and `end` correspond to the number of lines to
// /// display in the text box.
// struct BufWin {
//     start: usize,
//     end: usize,
//     len: usize,
//     tail: bool,
//     reader: Option<BufReader<File>>,
//     pub src: String,
// }
// 
// impl BufWin {
//     fn new(path: &str, tail: bool) -> anyhow::Result<Self> {
//         let reader = BufReader::new(File::open(path)?);
// 
//         Ok(BufWin {
//             start: 0,
//             end: 0,
//             len: 0,
//             tail,
//             reader: Some(reader),
//             src: String::new(),
//         })
//     }
// 
//     pub fn update(&mut self) -> anyhow::Result<()> {
//         if let Some(ref mut reader) = self.reader {
//             reader.read_line(&mut self.src)?;
//         }
//         Ok(())
//     }
// }
