use crate::{expression_level::table::ArgumentTermNode, package_level::TopStatementNode, IdentifierNode};
use alloc::vec::Vec;
use core::ops::Range;

pub struct LambdaArgumentNode {
    /// The raw string of the number.
    pub terms: Vec<TopStatementNode>,
    /// The range of the number.
    pub range: Range<usize>,
}
