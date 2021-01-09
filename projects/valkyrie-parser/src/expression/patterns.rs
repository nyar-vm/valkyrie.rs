use super::*;
use valkyrie_ast::PatternBlock;

impl ThisParser for PatternBlock {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pattern = BracketPattern::new("{", "}");
        let (state, terms) = pattern.consume(input.skip(ignore), ignore, PatternBranch::parse)?;
        state.finish(Self { branches: terms.body, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        unreachable!()
    }
}

impl ThisParser for PatternBranch {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, cond) = PatternCondition::parse(input)?;
        let (state, stmts) = state.match_repeats(pattern_statements)?;
        state.finish(Self { condition: cond, statements: PatternStatements { terms: stmts }, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(10);
        lisp += self.condition.lispify();
        for stmt in &self.statements.terms {
            lisp += stmt.lispify();
        }
        lisp
    }
}

static PREFIX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)(
    \b(case | when | type | else)\b
)"#,
    )
    .unwrap()
});

fn pattern_statements(input: ParseState) -> ParseResult<StatementNode> {
    if let ParseResult::Pending(_, _) = input.match_regex(&PREFIX, "PatternPrefix") {
        Err(StopBecause::ShouldNotBe { message: "PatternGuard", position: input.start_offset })?
    }
    StatementNode::parse(input)
}

impl ThisParser for PatternCondition {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, head) = input
            .begin_choice()
            .choose(|s| PatternCaseNode::parse(s).map_into())
            .choose(|s| PatternTypeNode::parse(s).map_into())
            .choose(|s| PatternWhenNode::parse(s).map_into())
            .choose(|s| PatternElseNode::parse(s).map_into())
            .end_choice()?;
        let (state, _) = state.skip(ignore).match_str(":")?;
        state.finish(head)
    }

    fn lispify(&self) -> Lisp {
        match self {
            Self::Case(v) => v.lispify(),
            Self::When(v) => v.lispify(),
            Self::Type(v) => v.lispify(),
            Self::Else(v) => v.lispify(),
        }
    }
}

impl ThisParser for ImplicitCaseNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, lhs) = LetPattern::parse(input)?;
        let (state, _) = state.skip(ignore).match_fn(parse_bind)?;
        let (state, rhs) = parse_expression_node(state.skip(ignore), ExpressionContext::default())?;
        state.finish(Self { pattern: lhs, body: rhs, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for PatternCaseNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("case")(input)?;
        let (state, pat) = state.skip(ignore).match_fn(ExpressionNode::parse)?;
        state.finish(Self { pattern: pat, guard: None, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        Lisp::keyword("match/case") + self.pattern.lispify()
    }
}
impl ThisParser for PatternTypeNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("type")(input)?;
        let (state, expr) = parse_expression_node(state.skip(ignore), ExpressionContext::in_type())?;
        state.finish(Self { pattern: expr, guard: None, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        Lisp::keyword("match/type") + self.pattern.lispify()
    }
}
impl ThisParser for PatternWhenNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("when")(input)?;
        let (state, expr) = parse_expression_node(state.skip(ignore), ExpressionContext::default())?;
        state.finish(Self { guard: expr, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        Lisp::keyword("match/when") + self.guard.lispify()
    }
}
impl ThisParser for PatternElseNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("else")(input)?;
        state.finish(Self { span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        Lisp::keyword("match/else")
    }
}

impl ThisParser for PatternGuard {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = parse_when(input)?;
        let (state, cond) = parse_expression_node(state.skip(ignore), ExpressionContext::default())?;
        state.finish(Self { condition: cond, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        Lisp::keyword("if") + self.condition.lispify()
    }
}

impl ThisParser for LetPattern {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .choose(|s| TuplePatternNode::parse(s).map_into())
            .choose(|s| ArgumentKeyNode::parse(s).map_into())
            .end_choice()
    }

    fn lispify(&self) -> Lisp {
        match self {
            Self::Tuple(s) => s.lispify(),
            Self::Symbol(s) => s.lispify(),
            Self::Class(s) => s.lispify(),
            Self::Array(s) => s.lispify(),
            Self::Union(s) => s.lispify(),
        }
    }
}

impl ThisParser for TuplePatternNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, name) = input.match_optional(NamePathNode::parse)?;
        let pat = if name.is_some() {
            BracketPattern::new("(", ")")
        }
        else {
            // need to check if it's a tuple or a parenthesized expression
            BracketPattern::new("(", ")").with_one_tailing(true)
        };
        let (state, terms) = pat.consume(state.skip(ignore), ignore, LetPattern::parse)?;
        state.finish(Self { bind: None, name, terms: terms.body, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(10);
        lisp += Lisp::keyword("pattern/tuple");
        if let Some(name) = &self.name {
            lisp += name.lispify();
        }
        for term in &self.terms {
            lisp += term.lispify();
        }
        lisp
    }
}

impl ThisParser for ArrayPatternNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        // let (state, name) = input.match_optional(NamePathNode::parse)?;
        let pat = BracketPattern::new("[", "]");
        let (state, terms) = pat.consume(input.skip(ignore), ignore, LetPattern::parse)?;
        state.finish(Self { bind: None, terms: terms.body, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for ClassPatternNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, name) = input.match_optional(NamePathNode::parse)?;
        todo!()
    }

    fn lispify(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for UnionPatternNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn lispify(&self) -> Lisp {
        todo!()
    }
}
