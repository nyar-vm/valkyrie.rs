use super::*;
use crate::{helpers::parse_comma, utils::parse_modifiers_lookahead};
use pex::StopBecause;
use valkyrie_ast::{ArgumentKeyNode, BareForPattern};

impl ThisParser for ForLoop {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("for")?;
        let (state, pattern) = state.skip(ignore).match_fn(PatternExpressionNode::parse)?;
        let (state, _) = state.skip(ignore).match_fn(parse_in)?;
        let (state, expr) = state.skip(ignore).match_fn(|s| parse_expression_node(s, ExpressionContext::default()))?;
        let (state, cond) = state.skip(ignore).match_optional(PatternGuard::parse)?;
        let (state, body) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        let (state, other) = state.skip(ignore).match_optional(ElseStatement::parse)?;
        state.finish(ForLoop { pattern, iterator: expr, condition: cond, body, no_run: other, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(10);
        terms.push(Lisp::keyword("for"));
        terms.push(self.pattern.as_lisp());
        terms.push(Lisp::keyword("in"));
        terms.push(self.iterator.as_lisp());
        if let Some(cond) = &self.condition {
            terms.push(Lisp::keyword("if"));
            terms.push(cond.as_lisp());
        }
        terms.push(Lisp::Any(self.body.terms.iter().map(|s| s.as_lisp()).collect()));
        terms.push(Lisp::keyword("else"));
        terms.push(Lisp::Any(self.no_run.iter().map(|s| s.as_lisp()).collect()));
        Lisp::Any(terms)
    }
}

impl ThisParser for BareForPattern {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, pats) = input.match_repeats(no_parentheses_tuple_term)?;
        if pats.is_empty() {
            StopBecause::missing_string("IDENTIFIER", input.start_offset)?
        }
        state.finish(Self { pattern: pats, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

#[inline]
fn no_parentheses_tuple_term(input: ParseState) -> ParseResult<ArgumentKeyNode> {
    let (state, (mods, id)) = parse_modifiers_lookahead(input, |s| s.eq("in"))?;
    let (state, _) = state.skip(ignore).match_optional(parse_comma)?;
    state.finish(ArgumentKeyNode { modifiers: mods, key: id })
}
