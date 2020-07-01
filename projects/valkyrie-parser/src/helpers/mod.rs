mod escaper;

pub use self::escaper::StringRewrite;

use crate::ThisParser;
use std::sync::LazyLock;
use valkyrie_ast::{IdentifierNode, NamePathNode};
use valkyrie_types::third_party::pex::{
    ParseResult,
    ParseResult::{Pending, Stop},
    ParseState, Regex, StopBecause,
};

pub static IGNORE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?x)(
    # whitespace
      \s
    # comments
    | \# [^\r\n]*
)*",
    )
    .unwrap()
});

/// Ignores whitespace and comments.
#[inline]
pub fn ignore<'i>(input: ParseState<'i>) -> ParseResult<&'i str> {
    match input.match_regex(&IGNORE, "IGNORE") {
        Pending(state, a) => input.advance_view(a.len()),
        Stop(_) => input.finish(""),
    }
}

/// `;` or `;;`
#[inline]
pub fn parse_eos(input: ParseState) -> ParseResult<bool> {
    if input.residual.starts_with(";;") {
        input.advance(";;").finish(false)
    }
    else if input.residual.starts_with(";") {
        input.advance(";").finish(true)
    }
    else {
        input.finish(false)
    }
}

/// `::` or `∷`
#[inline]
pub fn parse_name_join(input: ParseState) -> ParseResult<&str> {
    if input.residual.starts_with("::") {
        input.advance_view("::".len())
    }
    else if input.residual.starts_with('∷') {
        input.advance_view("∷".len())
    }
    else {
        StopBecause::missing_character('∷', input.start_offset)?
    }
}

/// `.` or `::` or `∷`
#[inline]
pub fn parse_name_join_dot(input: ParseState) -> ParseResult<&str> {
    if input.residual.starts_with('.') {
        input.advance_view(".".len())
    }
    else if input.residual.starts_with("::") {
        input.advance_view("::".len())
    }
    else if input.residual.starts_with('∷') {
        input.advance_view("∷".len())
    }
    else {
        StopBecause::missing_character('.', input.start_offset)?
    }
}

#[inline]
pub fn parse_any_name_path(input: ParseState) -> ParseResult<NamePathNode> {
    let (state, head) = IdentifierNode::parse(input)?;
    let (state, rest) = state.match_repeats(|s| {
        let (state, _) = s.skip(ignore).match_fn(parse_name_join_dot)?;
        state.skip(ignore).match_fn(IdentifierNode::parse)
    })?;
    let mut names = Vec::with_capacity(rest.len() + 1);
    names.push(head);
    names.extend(rest);
    state.finish(NamePathNode { names, span: state.away_from(input) })
}
