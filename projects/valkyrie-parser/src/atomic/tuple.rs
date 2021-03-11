use super::*;
use crate::{TupleKeyNode, TuplePairNode};
use valkyrie_ast::{ApplyArgument, ApplyCallNode, SubscriptCallNode, TupleKeyType, TupleTermNode};

impl TupleLiteralNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<TupleNode> {
        let mut errors = vec![];
        let mut terms = vec![];
        for x in &self.tuple_pair {
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
        Success { value: TupleNode { kind: Default::default(), terms, span: self.span.clone() }, diagnostics: errors }
    }
}
impl TuplePairNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<TupleTermNode> {
        let key = match &self.tuple_key {
            None => TupleKeyType::Nothing,
            Some(v) => v.build(ctx),
        };
        let value = self.main_expression.build(ctx)?;
        Success { value: TupleTermNode { key, value }, diagnostics: vec![] }
    }
}

impl TupleKeyNode {
    pub fn build(&self, ctx: &ProgramContext) -> TupleKeyType {
        match self {
            TupleKeyNode::Identifier(v) => TupleKeyType::Identifier(v.build(ctx)),
        }
    }
}

impl crate::TupleCallNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ApplyCallNode> {
        let monadic = self.op_and_then.is_some();
        // let terms = self.tuple_literal.build(ctx)?.terms;
        Success {
            value: ApplyCallNode {
                base: Default::default(),
                monadic,
                caller: Default::default(),
                arguments: None,
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}
