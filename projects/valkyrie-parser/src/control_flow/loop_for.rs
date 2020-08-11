use super::*;
use crate::{table::TupleNode, utils::parse_modifiers};
use valkyrie_ast::{ArgumentKeyNode, IdentifierNode, ModifierPart, TableKeyType::Identifier};
use valkyrie_types::third_party::pex::{BracketPattern, StopBecause};

impl ThisParser for ForLoopNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("for")?;
        let (state, pattern) = input.skip(ignore).match_fn(PatternType::parse)?;
        let (state, cond) = state.skip(ignore).match_fn(parse_condition)?;

        state.finish(ForLoopNode {
            pattern: PatternType::Case,
            condition: ConditionType::AlwaysTrue,
            body: vec![],
            r#else: vec![],
            span: get_span(input, state),
        })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

fn parse_condition(input: ParseState) -> ParseResult<ConditionType> {
    let (state, _) = input.match_str("if")?;
    let (state, cond) = state.skip(ignore).match_fn(ConditionType::parse)?;
    state.finish(cond)
}

impl ThisParser for PatternType {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

fn parentheses_tuple(input: ParseState) -> ParseResult<PatternType> {
    let pat = BracketPattern::new("(", ")").with_one_tailing(true);
    parse_modifiers()
}

/// term
/// term,
fn no_parentheses_tuple(input: ParseState) -> ParseResult<PatternType> {
    let (state, parts) = input.match_repeats(no_parentheses_tuple_term)?;
    state.finish(PatternType::Tuple(parts))
}

fn no_parentheses_tuple_term(input: ParseState) -> ParseResult<ArgumentKeyNode> {
    let (state, mut names) = input.match_repeats(|s| {
        let (state, name) = s.skip(ignore).match_fn(IdentifierNode::parse)?;
        if name.name.eq("in") {
            StopBecause::custom_error("Expected identifier, found `in`", s.start_offset, s.start_offset + 2)?
        }
        state.finish(name)
    })?;
    let (finally, _) = state.skip(ignore).match_optional(|s| s.match_str(","))?;
    let name = names.pop();
    match name {
        Some(s) => finally.finish(ArgumentKeyNode { modifiers: names, key: s }),
        None => StopBecause::custom_error("Expected identifier, found `,`", input.start_offset, input.start_offset + 2)?,
    }
}
