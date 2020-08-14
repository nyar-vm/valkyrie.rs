use super::*;

impl ThisParser for ForLoopNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("for")?;
        let (state, pattern) = state.skip(ignore).match_fn(PatternType::parse)?;
        let (state, _) = state.skip(ignore).match_fn(parse_in)?;
        let (state, expr) = state.skip(ignore).match_fn(|s| {
            parse_expression_node(s, ExpressionContext { type_level: false, allow_newline: true, allow_curly: false })
        })?;
        let (state, cond) = state.skip(ignore).match_optional(parse_condition)?;
        let (state, body) = state.skip(ignore).match_fn(FunctionBodyPart::parse)?;
        let (state, other) = state.skip(ignore).match_optional(ElsePart::parse)?;
        state.finish(ForLoopNode {
            pattern,
            iterator: expr,
            condition: cond.unwrap_or(ConditionType::AlwaysTrue),
            body: body.body.to_vec(),
            r#else: other.map(|s| s.body.to_vec()).unwrap_or_default(),
            span: get_span(input, state),
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(10);
        terms.push(Lisp::keyword("for"));
        terms.push(self.pattern.as_lisp());
        terms.push(Lisp::keyword("in"));
        terms.push(self.iterator.as_lisp());
        terms.push(Lisp::keyword("if"));
        terms.push(self.condition.as_lisp());
        terms.push(Lisp::Any(self.body.iter().map(|s| s.as_lisp()).collect()));
        terms.push(Lisp::keyword("else"));
        terms.push(Lisp::Any(self.r#else.iter().map(|s| s.as_lisp()).collect()));
        Lisp::Any(terms)
    }
}

fn parse_condition(input: ParseState) -> ParseResult<ConditionType> {
    let (state, _) = input.match_str("if")?;
    let (state, cond) = state.skip(ignore).match_fn(ConditionType::parse)?;
    state.finish(cond)
}

impl ThisParser for PatternType {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(no_parentheses_tuple).or_else(parentheses_tuple).end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            PatternType::Tuple(s) => Lisp::Any(s.iter().map(|s| s.as_lisp()).collect()),
            PatternType::Case => Lisp::keyword("case"),
        }
    }
}

fn parentheses_tuple(input: ParseState) -> ParseResult<PatternType> {
    let pat = BracketPattern::new("(", ")").with_one_tailing(true);
    let (state, terms) = pat.consume(input, ignore, ArgumentKeyNode::parse)?;
    state.finish(PatternType::Tuple(terms.body))
}

/// term
/// term,
fn no_parentheses_tuple(input: ParseState) -> ParseResult<PatternType> {
    let (state, parts) = input.match_repeats(no_parentheses_tuple_term)?;
    if parts.is_empty() {
        StopBecause::missing_string("IDENTIFIER", input.start_offset)?
    }
    state.finish(PatternType::Tuple(parts))
}

fn no_parentheses_tuple_term(input: ParseState) -> ParseResult<ArgumentKeyNode> {
    let (state, (mods, id)) = parse_modifiers(input, |s| s.eq("in"))?;
    let (state, _) = state.skip(ignore).match_optional(parse_comma)?;
    state.finish(ArgumentKeyNode { modifiers: mods, key: id })
}
