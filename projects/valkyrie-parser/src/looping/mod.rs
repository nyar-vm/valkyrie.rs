use std::ops::Range;

pub struct ForStatementNode<E, B> {
    pub iterator: E,
    pub body: B,
    pub range: Range<usize>,
}
