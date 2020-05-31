use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
    hash::Hash,
    ops::Range,
};
use syntax_error::{Cache, Source};
use url::Url;

#[inline(always)]
pub fn small_range(range: &Range<usize>) -> Range<u32> {
    Range { start: range.start as u32, end: range.end as u32 }
}

#[inline(always)]
pub fn take_range(range: &Range<u32>) -> Range<usize> {
    Range { start: range.start as usize, end: range.end as usize }
}

pub type FileID = u32;

#[derive(Copy, Clone, Debug, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileSpan {
    file: u32,
    start: u32,
    end: u32,
}

pub struct FileStorage {
    files: BTreeMap<u32, Url>,
}

impl Cache<u32> for FileStorage {
    fn fetch(&mut self, id: &u32) -> Result<&Source, Box<dyn Debug + '_>> {
        todo!()
    }

    fn display<'a>(&self, id: &'a u32) -> Option<Box<dyn Display + 'a>> {
        todo!()
    }
}

impl FileSpan {
    /// Creates a new file span.
    pub fn new(range: Range<usize>) -> Self {
        Self { file: 0, start: range.start as u32, end: range.end as u32 }
    }
    /// Attach file id to the span.
    pub fn with_file(mut self, file: FileID) -> Self {
        self.file = file;
        self
    }
    pub fn get_file(&self) -> FileID {
        self.file
    }
    pub fn get_range(&self) -> Range<usize> {
        Range { start: self.start as usize, end: self.end as usize }
    }
}

impl PartialEq for FileSpan {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
