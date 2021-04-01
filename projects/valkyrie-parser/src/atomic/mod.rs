use crate::{
    helpers::ProgramContext, AtomicNode, RangeLiteralNode, SpecialNode, SubscriptAxisNode, SubscriptOnlyNode,
    SubscriptRangeNode, TupleKeyNode, TupleLiteralNode, TuplePairNode, TupleTermsNode,
};
use nyar_error::{Failure, Success, Validate, Validation};
use valkyrie_ast::{
    ApplyCallNode, ArgumentsList, BooleanNode, ConstructNewNode, ConstructObjectNode, ExpressionType, IdentifierNode,
    NamePathNode, NumberLiteralNode, RangeKind, RangeNode, RangeTermNode, StringLiteralNode, StringTextNode, TupleNode,
    TupleTermNode,
};
use yggdrasil_rt::YggdrasilNode;
mod bytes;
mod identifier;
mod number;
mod range;
mod string;
mod tuple;

mod create_new;
mod create_object;

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
            AtomicNode::NewStatement(v) => v.build(ctx).into(),
            AtomicNode::ObjectStatement(v) => v.build(ctx).into(),
        };
        Success { value, diagnostics: vec![] }
    }
}
