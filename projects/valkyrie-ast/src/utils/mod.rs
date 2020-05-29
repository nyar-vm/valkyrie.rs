use std::{
    hash::{Hash, Hasher},
    ops::Range,
    path::Path,
};

#[inline(always)]
pub fn small_range(range: &Range<usize>) -> Range<u32> {
    Range { start: range.start as u32, end: range.end as u32 }
}

#[inline(always)]
pub fn take_range(range: &Range<u32>) -> Range<usize> {
    Range { start: range.start as usize, end: range.end as usize }
}

#[derive(Copy, Clone, Debug, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileSpan {
    file: u32,
    start: u32,
    end: u32,
}

impl FileSpan {
    /// Creates a new file span.
    pub fn new(range: Range<usize>) -> Self {
        Self { file, start: range.start as u32, end: range.end as u32 }
    }
    /// Attach file id to the span.
    pub fn with_file(mut self, file: u32) -> Self {
        self.file = file;
        self
    }
    pub fn get_file(&self) -> u32 {
        self.file as u32
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
