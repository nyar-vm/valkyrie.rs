use super::*;

/// `loop { }`
pub struct LoopStatementNode<B> {
    pub body: B,
    pub range: Range<usize>,
}
