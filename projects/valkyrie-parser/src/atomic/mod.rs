use crate::{
    helpers::ProgramContext, AtomicNode, RangeLiteralNode, SpecialNode, SubscriptAxisNode, SubscriptOnlyNode,
    SubscriptRangeNode, TupleLiteralNode,
};
use nyar_error::{Failure, Success, Validate, Validation};
use valkyrie_ast::{
    BooleanNode, ExpressionType, IdentifierNode, NamePathNode, NumberLiteralNode, RangeKind, RangeNode, RangeTermNode,
    StringLiteralNode, StringTextNode, TupleNode,
};
use yggdrasil_rt::YggdrasilNode;
mod bytes;
mod identifier;
mod number;
mod range;
mod string;
mod tuple;

impl AtomicNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ExpressionType> {
        let value = match self {
            AtomicNode::Special(v) => v.build(),
            AtomicNode::Integer(v) => v.build().into(),
            AtomicNode::Namepath(v) => v.build(ctx).into(),
            AtomicNode::ProceduralCall(_) => {
                todo!()
            }
            AtomicNode::RangeLiteral(v) => v.build(ctx)?.into(),
            AtomicNode::TupleLiteral(v) => v.build(ctx)?.into(),
            AtomicNode::TextLiteral(v) => v.build(ctx).into(),
        };
        Success { value, diagnostics: vec![] }
    }
}
