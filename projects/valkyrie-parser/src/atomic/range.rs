use super::*;
use crate::{MainExpressionNode, SubscriptAxisNode, SubscriptOnlyNode, SubscriptRangeNode};
use nyar_error::{Failure, Validate};
use valkyrie_ast::{ArgumentTermNode, ArrayKind, ArrayNode, ArrayTermNode, TupleKeyType};

impl RangeLiteralNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ArrayNode> {
        let mut errors = vec![];
        let mut terms = vec![];
        for x in &self.subscript_axis {
            match x.build(ctx) {
                Success { value, diagnostics } => {
                    terms.push(value);
                    errors.extend(diagnostics);
                }
                Failure { fatal, diagnostics } => {
                    errors.push(fatal);
                    errors.extend(diagnostics);
                }
            }
        }
        Success { value: ArrayNode { kind: ArrayKind::Ordinal, terms, span: self.span.clone() }, diagnostics: errors }
    }
}

impl SubscriptAxisNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ArrayTermNode> {
        match self {
            SubscriptAxisNode::SubscriptOnly(v) => v.build(ctx),
            SubscriptAxisNode::SubscriptRange(v) => v.build(ctx),
        }
    }
}

impl SubscriptOnlyNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ArrayTermNode> {
        self.index.build(ctx).map(|v| ArrayTermNode::Index { index: v.body })
    }
}

impl SubscriptRangeNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ArrayTermNode> {
        let head = match &self.head {
            Some(s) => Some(s.build(ctx)?.body),
            None => None,
        };
        let tail = match &self.tail {
            Some(s) => Some(s.build(ctx)?.body),
            None => None,
        };
        let step = match &self.step {
            Some(s) => Some(s.build(ctx)?.body),
            None => None,
        };
        Success { value: ArrayTermNode::Range { head, tail, step }, diagnostics: vec![] }
    }
}
