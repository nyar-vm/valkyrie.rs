use crate::{
    helpers::ProgramContext, LeadingNode, RangeLiteralNode, SpecialNode, SubscriptAxisNode, SubscriptOnlyNode,
    SubscriptRangeNode, TupleKeyNode, TupleLiteralNode, TupleLiteralStrictNode, TuplePairNode, TupleTermsNode,
};
use nyar_error::{Failure, Success, Validate, Validation};
use valkyrie_ast::{
    ApplyCallNode, ArgumentsList, BooleanNode, ConstructNewNode, ConstructObjectNode, ExpressionType, IdentifierNode,
    NamePathNode, NumberLiteralNode, RangeKind, RangeNode, RangeTermNode, StringLiteralNode, StringTextNode, TupleKind,
    TupleNode, TupleTermNode,
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
mod create_try;

impl LeadingNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ExpressionType> {
        let value = match self {
            Self::Special(v) => v.build(),
            Self::Number(v) => v.build(ctx)?.into(),
            Self::Namepath(v) => v.build(ctx).into(),
            Self::ProceduralCall(_) => {
                todo!()
            }
            Self::RangeLiteral(v) => v.build(ctx)?.into(),
            Self::TupleLiteralStrict(v) => v.build(ctx)?.into(),
            Self::TextLiteral(v) => v.build(ctx).into(),
        };
        Success { value, diagnostics: vec![] }
    }
}
