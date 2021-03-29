use super::*;
use crate::{TupleKeyNode, TuplePairNode};
use nyar_error::NyarError;
use std::{num::NonZeroU64, str::FromStr};
use valkyrie_ast::{ApplyCallNode, ArgumentsList, SubscriptCallNode, TupleKeyType, TupleTermNode};

impl TupleLiteralNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<TupleNode> {
        let mut errors = vec![];
        let mut terms = vec![];
        for x in &self.tuple_pair {
            x.build(ctx).append(&mut terms, &mut errors)
        }
        Success { value: TupleNode { kind: Default::default(), terms, span: self.span.clone() }, diagnostics: errors }
    }
    pub fn build_terms(&self, ctx: &ProgramContext) -> Validation<ArgumentsList> {
        Success { value: ArgumentsList { terms: self.build(ctx)?.terms, span: Default::default() }, diagnostics: vec![] }
    }
}

impl TuplePairNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<TupleTermNode> {
        let key = match &self.tuple_key {
            None => TupleKeyType::Nothing,
            Some(v) => v.build(ctx)?,
        };
        let value = self.main_expression.build(ctx)?;
        Success { value: TupleTermNode { key, value }, diagnostics: vec![] }
    }
}

impl TupleKeyNode {
    pub fn build(&self, ctx: &ProgramContext) -> Result<TupleKeyType, NyarError> {
        match self {
            Self::Identifier(v) => Ok(TupleKeyType::Identifier(v.build(ctx))),
            Self::Integer(v) => {
                let n = u64::from_str(&v.text)?;
                match NonZeroU64::new(n) {
                    Some(n) => Ok(TupleKeyType::String(n)),
                    None => Err(NyarError::syntax_error(
                        "Tuple key cannot be zero",
                        ctx.file.with_range(v.get_range().unwrap_or_default().clone()),
                    )),
                }
            }
        }
    }
}

impl crate::TupleCallNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ApplyCallNode> {
        let monadic = self.op_and_then.is_some();
        let arguments = match &self.tuple_literal {
            Some(s) => Some(s.build_terms(ctx)?),
            None => None,
        };
        Success {
            value: ApplyCallNode { monadic, caller: Default::default(), arguments, body: None, span: self.span.clone() },
            diagnostics: vec![],
        }
    }
}
