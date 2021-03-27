use super::*;
use crate::KwWhileNode;

impl crate::WhileStatementNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<WhileLoop> {
        Success {
            value: WhileLoop {
                kind: self.kw_while.build(),
                condition: WhileConditionNode::Unconditional,
                then_body: Default::default(),
                otherwise: None,
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
