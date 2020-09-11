use super::*;

impl ThisParser for PatternCondition {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for PatternGuard {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = parse_when(input)?;
        let (state, cond) = parse_expression_node(state.skip(ignore), ExpressionContext::default())?;
        state.finish(Self { condition: cond, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::Any(vec![Lisp::keyword("if"), self.condition.as_lisp()])
    }
}

impl ThisParser for PatternExpression {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(no_parentheses_tuple).or_else(parentheses_tuple).end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            PatternExpression::Tuple(s) => Lisp::Any(s.iter().map(|s| s.as_lisp()).collect()),
            PatternExpression::Case => Lisp::keyword("case"),
        }
    }
}

fn parentheses_tuple(input: ParseState) -> ParseResult<PatternExpression> {
    let pat = BracketPattern::new("(", ")").with_one_tailing(true);
    let (state, terms) = pat.consume(input, ignore, ArgumentKeyNode::parse)?;
    state.finish(PatternExpression::Tuple(terms.body))
}

/// term
/// term,
fn no_parentheses_tuple(input: ParseState) -> ParseResult<PatternExpression> {
    let (state, parts) = input.match_repeats(no_parentheses_tuple_term)?;
    if parts.is_empty() {
        StopBecause::missing_string("IDENTIFIER", input.start_offset)?
    }
    state.finish(PatternExpression::Tuple(parts))
}

fn no_parentheses_tuple_term(input: ParseState) -> ParseResult<ArgumentKeyNode> {
    let (state, (mods, id)) = parse_modifiers_lookahead(input, |s| s.eq("in"))?;
    let (state, _) = state.skip(ignore).match_optional(parse_comma)?;
    state.finish(ArgumentKeyNode { modifiers: mods, key: id })
}
