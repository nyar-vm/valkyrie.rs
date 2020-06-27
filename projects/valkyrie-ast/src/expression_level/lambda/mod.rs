use crate::{expression_level::table::ArgumentTermNode, package_level::TopStatementNode, IdentifierNode};
use std::ops::Range;

pub struct LambdaArgumentNode {
    /// The raw string of the number.
    pub terms: Vec<TopStatementNode>,
    /// The range of the number.
    pub range: Range<usize>,
}
