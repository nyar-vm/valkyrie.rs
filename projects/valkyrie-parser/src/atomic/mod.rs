use crate::{helpers::ProgramContext, AtomicNode};
use nyar_error::{Success, Validation};
use valkyrie_ast::{BooleanNode, ExpressionNode, ExpressionType, NumberLiteralNode};

// mod bytes;
mod identifier;
mod namepath;
mod number;

impl AtomicNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ExpressionType> {
        let value = match self {
            AtomicNode::Boolean(v) => v.build().into(),
            AtomicNode::Integer(v) => v.build().into(),
            AtomicNode::Namepath(v) => v.build(ctx).into(),
            AtomicNode::ProceduralCall(_) => {
                todo!()
            }
            AtomicNode::RangeLiteral(_) => {
                todo!()
            }
            AtomicNode::TupleLiteral(_) => {
                todo!()
            }
        };
        Success { value, diagnostics: vec![] }
    }
}
