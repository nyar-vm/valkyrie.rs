use std::ops::Range;

#[inline(always)]
pub fn small_range(range: &Range<usize>) -> Range<u32> {
    Range { start: range.start as u32, end: range.end as u32 }
}

#[inline(always)]
pub fn take_range(range: &Range<u32>) -> Range<usize> {
    Range { start: range.start as usize, end: range.end as usize }
}
