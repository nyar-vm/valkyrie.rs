use super::*;
use nyar_error::{NyarError, SyntaxError};

impl crate::TupleLiteralStrictNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<TupleNode> {
        let mut errors = vec![];
        let mut terms = vec![];
        for x in &self.tuple_pair {
            x.build(ctx).append(&mut terms, &mut errors)
        }
        Success {
            value: TupleNode { kind: TupleKind::Tuple, terms: ArgumentsList { terms }, span: self.span.clone() },
            diagnostics: errors,
        }
    }
}

impl crate::TupleLiteralNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<TupleNode> {
        self.tuple_terms.build(ctx).map(|terms| TupleNode { kind: Default::default(), terms, span: self.span.clone() })
    }
}

impl crate::TupleTermsNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ArgumentsList> {
        let mut errors = vec![];
        let mut terms = vec![];
        for x in &self.tuple_pair {
            x.build(ctx).append(&mut terms, &mut errors)
        }
        Success { value: ArgumentsList { terms }, diagnostics: errors }
    }
}

impl crate::TuplePairNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<TupleTermNode> {
        let key = match &self.tuple_key {
            Some(v) => Some(v.build(ctx)?),
            None => None,
        };
        let value = self.main_expression.build(ctx)?;
        Success { value: TupleTermNode { key, value }, diagnostics: vec![] }
    }
}

impl crate::TupleKeyNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<IdentifierNode, NyarError> {
        match self {
            Self::Identifier(v) => Ok(v.build(ctx)),
            Self::TextRaw(v) => Ok(v.build_id(ctx)),
            Self::Integer(v) => {
                Err(SyntaxError::new("tuple key cannot be a number").with_range(&v.span).with_file(ctx.file).into())
            }
        }
    }
}

impl crate::TupleCallNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ApplyCallNode> {
        let monadic = self.op_and_then.is_some();
        let arguments = match &self.tuple_literal {
            Some(s) => s.build(ctx)?.terms,
            None => ArgumentsList { terms: vec![] },
        };
        Success {
            value: ApplyCallNode { monadic, caller: Default::default(), arguments, body: None, span: self.span.clone() },
            diagnostics: vec![],
        }
    }
}
impl crate::InlineTupleCallNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ApplyCallNode> {
        let monadic = self.op_and_then.is_some();
        let arguments = self.tuple_literal.build(ctx)?.terms;
        Success {
            value: ApplyCallNode { monadic, caller: Default::default(), arguments, body: None, span: self.span.clone() },
            diagnostics: vec![],
        }
    }
}
