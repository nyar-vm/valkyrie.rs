use super::*;
mod display;

/// A number literal.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewNode<E> {
    pub index0: bool,
    /// The raw string of the number.
    pub base: E,
    /// The raw string of the number.
    pub terms: Vec<ViewTermNode<E>>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// A number literal.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ViewTermNode<E> {
    /// `array[index]`, also can be a call_index `array[[1, 2, 3]]`
    Index(E),
    /// `array[start:end:step]`
    Range(ViewRangeNode<E>),
}

///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewRangeNode<E> {
    /// The raw string of the number.
    pub start: Option<E>,
    /// The unit of the number, if any.
    pub end: Option<E>,
    /// The unit of the number, if any.
    pub step: Option<E>,
    /// The range of the number.
    pub range: Range<usize>,
}

impl<E> ViewTermNode<E> {
    pub fn indexed(index: E) -> Self {
        Self::Index(index)
    }
    pub fn ranged(start: Option<E>, end: Option<E>, step: Option<E>, range: Range<usize>) -> Self {
        Self::Range(ViewRangeNode { start, end, step, range })
    }
}
