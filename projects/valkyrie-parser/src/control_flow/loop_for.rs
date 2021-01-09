use super::*;
use crate::{helpers::parse_comma, utils::parse_modifiers_lookahead};
use pex::StopBecause;
use valkyrie_ast::{ArgumentKeyNode, ForBarePattern, OtherwiseStatement};

impl ThisParser for ForLoop {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("for")?;
        let (state, pattern) = state.skip(ignore).match_fn(for_pattern)?;
        let (state, _) = state.skip(ignore).match_fn(parse_in)?;
        let (state, expr) = state.skip(ignore).match_fn(|s| parse_expression_node(s, ExpressionContext::default()))?;
        let (state, cond) = state.skip(ignore).match_optional(PatternGuard::parse)?;
        let (state, body) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        let (state, other) = state.skip(ignore).match_optional(OtherwiseStatement::parse)?;
        state.finish(ForLoop {
            pattern,
            iterator: expr,
            condition: cond,
            then_body: body,
            else_body: other,
            span: get_span(input, state),
        })
    }
}

fn for_pattern(input: ParseState) -> ParseResult<LetPattern> {
    input
        .begin_choice()
        .choose(|s| ForBarePattern::parse(s).map_inner(|s| s.as_pattern_expression()))
        .choose(LetPattern::parse)
        .end_choice()
}

impl ThisParser for ForBarePattern {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, pats) = input.match_repeats(no_parentheses_tuple_term)?;
        if pats.is_empty() {
            StopBecause::missing_string("IDENTIFIER", input.start_offset)?
        }
        state.finish(Self { pattern: pats, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        unreachable!()
    }
}

#[inline]
fn no_parentheses_tuple_term(input: ParseState) -> ParseResult<ArgumentKeyNode> {
    let (state, (mods, id)) = parse_modifiers_lookahead(input, |s| s.eq("in"))?;
    let (state, _) = state.skip(ignore).match_optional(parse_comma)?;
    state.finish(ArgumentKeyNode { modifiers: mods, key: id })
}
