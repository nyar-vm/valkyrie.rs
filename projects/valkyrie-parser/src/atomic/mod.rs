use crate::{helpers::ProgramContext, AtomicNode, RangeCallNode, RangeLiteralNode};
use nyar_error::{Success, Validation};
use valkyrie_ast::{BooleanNode, ExpressionNode, ExpressionType, NumberLiteralNode};

mod bytes;
mod identifier;
mod number;
mod range;
mod string;
mod tuple;

impl AtomicNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ExpressionType> {
        let value = match self {
            AtomicNode::Special(v) => v.build().into(),
            AtomicNode::Integer(v) => v.build().into(),
            AtomicNode::Namepath(v) => v.build(ctx).into(),
            AtomicNode::ProceduralCall(_) => {
                todo!()
            }
            AtomicNode::RangeLiteral(v) => v.build(ctx)?.into(),
            AtomicNode::TupleLiteral(_) => {
                todo!()
            }
        };
        Success { value, diagnostics: vec![] }
    }
}
