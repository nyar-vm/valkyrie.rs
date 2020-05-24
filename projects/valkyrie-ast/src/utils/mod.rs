use std::{num::NonZeroU32, ops::Range, sync::Arc};
use url::Url;

#[derive(Clone, Debug)]
pub struct FileSpan {
    range: Range<u32>,
    file: Option<NonZeroU32>,
}

impl FileSpan {
    pub fn new(range: &Range<usize>) -> Self {
        Self { range: Range { start: range.start as u32, end: range.end as u32 }, file: None }
    }
    pub fn with_id(mut self, id: u32) -> Self {
        Self { range: Default::default(), file: NonZeroU32::new(id) }
    }
}

#[inline(always)]
pub fn small_range(range: &Range<usize>) -> Range<u32> {
    Range { start: range.start as u32, end: range.end as u32 }
}
