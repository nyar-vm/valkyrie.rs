use super::*;

impl crate::TupleLiteralStrictNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<TupleNode> {
        let mut terms = vec![];
        for term in &self.tuple_pair {
            match term.build(ctx) {
                Ok(o) => terms.push(o),
                Err(e) => ctx.add_error(e),
            }
        }
        Ok(TupleNode { kind: TupleKind::Tuple, terms: ArgumentsList { terms }, span: self.span.clone() })
    }
}

impl crate::TupleLiteralNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> TupleNode {
        TupleNode { kind: Default::default(), terms: self.tuple_terms.build(ctx), span: self.span.clone() }
    }
}

impl crate::TupleTermsNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> ArgumentsList {
        let mut list = ArgumentsList::new(self.tuple_pair.len());
        for term in &self.tuple_pair {
            match term.build(ctx) {
                Ok(o) => list.terms.push(o),
                Err(e) => ctx.add_error(e),
            }
        }
        list
    }
}

impl crate::TuplePairNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ArgumentTerm> {
        let key = match &self.tuple_key {
            Some(v) => v.build(ctx),
            None => ArgumentKey::Nothing,
        };
        Ok(ArgumentTerm { modifiers: Default::default(), key, value: self.main_expression.build(ctx)? })
    }
}

impl crate::TupleKeyNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> ArgumentKey {
        match self {
            Self::Identifier(v) => ArgumentKey::Symbol(v.build(ctx)),
            Self::TextRaw(v) => ArgumentKey::Symbol(v.build_id(ctx)),
            Self::Integer(v) => {
                ctx.add_error(
                    SyntaxError::new("tuple key cannot be a number")
                        .with_hint("Expect a symbol")
                        .with_range(&v.span)
                        .with_file(ctx.file),
                );

                ArgumentKey::Nothing
            }
        }
    }
}

impl crate::TupleCallNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ApplyCallNode> {
        let monadic = self.op_and_then.is_some();
        let arguments = match &self.tuple_literal {
            Some(s) => s.build(ctx).terms,
            None => ArgumentsList { terms: vec![] },
        };
        Ok(ApplyCallNode { monadic, caller: Default::default(), arguments, body: None, span: self.span.clone() })
    }
}
impl crate::InlineTupleCallNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ApplyCallNode> {
        let monadic = self.op_and_then.is_some();
        let arguments = self.tuple_literal.build(ctx).terms;
        Ok(ApplyCallNode { monadic, caller: Default::default(), arguments, body: None, span: self.span.clone() })
    }
}
