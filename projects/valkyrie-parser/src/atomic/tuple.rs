use super::*;

impl TupleLiteralStrictNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<TupleNode> {
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

impl TupleLiteralNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<TupleNode> {
        self.tuple_terms.build(ctx).map(|terms| TupleNode { kind: Default::default(), terms, span: self.span.clone() })
    }
}

impl TupleTermsNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ArgumentsList> {
        let mut errors = vec![];
        let mut terms = vec![];
        for x in &self.tuple_pair {
            x.build(ctx).append(&mut terms, &mut errors)
        }
        Success { value: ArgumentsList { terms }, diagnostics: errors }
    }
}

impl TuplePairNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<TupleTermNode> {
        let key = match &self.tuple_key {
            Some(v) => Some(v.build(ctx)),
            None => None,
        };
        let value = self.main_expression.build(ctx)?;
        Success { value: TupleTermNode { key, value }, diagnostics: vec![] }
    }
}

impl TupleKeyNode {
    pub fn build(&self, ctx: &ProgramContext) -> IdentifierNode {
        match self {
            Self::Identifier(v) => v.build(ctx),
            Self::TextRaw(v) => v.build_id(ctx),
        }
    }
}

impl crate::TupleCallNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ApplyCallNode> {
        let monadic = self.op_and_then.is_some();
        let arguments = match &self.tuple_literal {
            Some(s) => Some(s.build(ctx)?.terms),
            None => None,
        };
        Success {
            value: ApplyCallNode { monadic, caller: Default::default(), arguments, body: None, span: self.span.clone() },
            diagnostics: vec![],
        }
    }
}
