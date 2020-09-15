use super::*;
use pex::Regex;
use std::sync::LazyLock;
use valkyrie_ast::{AnnotationList, AnnotationNode, DocumentationNode, PatternBranch, PatternStatements, StatementNode};

impl ThisParser for PatternBranch {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, cond) = PatternCondition::parse(input)?;
        let (state, stmts) = state.match_repeats(pattern_statements)?;
        state.finish(Self { condition: cond, statements: PatternStatements { terms: stmts }, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
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
            .or_else(|s| PatternCaseNode::parse(s).map_into())
            .or_else(|s| PatternTypeNode::parse(s).map_into())
            .or_else(|s| PatternWhenNode::parse(s).map_into())
            .or_else(|s| PatternElseNode::parse(s).map_into())
            .end_choice()?;
        let (state, _) = state.skip(ignore).match_str(":")?;
        state.finish(head)
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl ThisParser for PatternCaseNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("case")(input)?;
        let (state, pat) = state.skip(ignore).match_fn(ExpressionNode::parse)?;
        state.finish(Self { pattern: pat, guard: None, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}
impl ThisParser for PatternTypeNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("type")(input)?;
        let (state, expr) = parse_expression_node(state.skip(ignore), ExpressionContext::in_type())?;
        state.finish(Self { pattern: expr, guard: None, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}
impl ThisParser for PatternWhenNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("when")(input)?;
        let (state, expr) = parse_expression_node(state.skip(ignore), ExpressionContext::default())?;
        state.finish(Self { guard: expr, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}
impl ThisParser for PatternElseNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("else")(input)?;
        state.finish(Self { span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
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
