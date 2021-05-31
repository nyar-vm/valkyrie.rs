use super::*;

impl crate::TupleLiteralStrictNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<TupleNode> {
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
    pub fn build(&self, ctx: &mut ProgramState) -> Result<TupleNode> {
        self.tuple_terms.build(ctx).map(|terms| TupleNode { kind: Default::default(), terms, span: self.span.clone() })
    }
}

impl crate::TupleTermsNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<ArgumentsList> {
        let mut terms = vec![];
        for term in &self.tuple_pair {
            match term.build(ctx) {
                Ok(o) => terms.push(o),
                Err(e) => ctx.add_error(e),
            }
        }
        Ok(ArgumentsList { terms })
    }
}

impl crate::TuplePairNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<TupleTermNode> {
        let key = match &self.tuple_key {
            Some(v) => Some(v.build(ctx)?),
            None => None,
        };
        let value = self.main_expression.build(ctx)?;
        Ok(TupleTermNode { key, value })
    }
}

impl crate::TupleKeyNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<IdentifierNode> {
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
    pub fn build(&self, ctx: &mut ProgramState) -> Result<ApplyCallNode> {
        let monadic = self.op_and_then.is_some();
        let arguments = match &self.tuple_literal {
            Some(s) => s.build(ctx)?.terms,
            None => ArgumentsList { terms: vec![] },
        };
        Ok(ApplyCallNode { monadic, caller: Default::default(), arguments, body: None, span: self.span.clone() })
    }
}
impl crate::InlineTupleCallNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<ApplyCallNode> {
        let monadic = self.op_and_then.is_some();
        let arguments = self.tuple_literal.build(ctx)?.terms;
        Ok(ApplyCallNode { monadic, caller: Default::default(), arguments, body: None, span: self.span.clone() })
    }
}
