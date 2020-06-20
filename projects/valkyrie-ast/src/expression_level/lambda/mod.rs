use crate::{expression_level::table::ArgumentTermNode, IdentifierNode};
use std::ops::Range;

pub struct LambdaArgumentNode<E1, E2> {
    /// The raw string of the number.
    pub terms: Vec<ArgumentTermNode<IdentifierNode, E1, E2>>,
    /// The range of the number.
    pub range: Range<usize>,
}
