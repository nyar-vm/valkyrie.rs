use super::*;
use crate::{ContinuationNode, KwWhileNode};
use valkyrie_ast::StatementBlock;

impl crate::WhileStatementNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<WhileLoop> {
        Success {
            value: WhileLoop {
                kind: self.kw_while.build(),
                condition: WhileConditionNode::Unconditional,
                then: self.continuation.build(ctx)?,
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}

impl KwWhileNode {
    pub fn build(&self) -> WhileLoopKind {
        match self {
            Self::Until => WhileLoopKind::Until,
            Self::While => WhileLoopKind::While,
        }
    }
}

impl ContinuationNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<StatementBlock> {
        let mut terms = vec![];
        let mut diagnostics = vec![];
        for term in &self.main_statement {
            term.build(ctx).append(&mut terms, &mut diagnostics)
        }
        Success { value: StatementBlock { terms, span: self.span.clone() }, diagnostics }
    }
}
