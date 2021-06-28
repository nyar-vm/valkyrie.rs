use crate::{IdentifierNode, StatementKind};
use alloc::{vec, vec::Vec};

#[derive(Debug)]
pub struct LoopStatement {
    pub label: Option<IdentifierNode>,
    pub terms: Vec<StatementKind>,
}

#[derive(Debug)]
pub struct LoopContinuation {
    r#continue: Vec<StatementKind>,
    r#break: Vec<StatementKind>,
}

impl LoopContinuation {
    /// create a new loop
    pub fn new(v: Vec<StatementKind>) -> Self {
        Self { r#continue: vec![], r#break: vec![] }
    }
}
