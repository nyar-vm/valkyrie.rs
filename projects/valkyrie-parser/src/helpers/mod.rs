mod escaper;

pub use self::escaper::StringRewrite;

use crate::{utils::get_span, ThisParser};
use pex::{
    ParseResult,
    ParseResult::{Pending, Stop},
    ParseState, Regex, StopBecause,
};
use std::sync::LazyLock;
use valkyrie_ast::{IdentifierNode, NamePathNode};

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
    let state = input.skip(ignore);

    if state.residual.starts_with(";;") {
        state.advance(";;").finish(false)
    }
    else if state.residual.starts_with(";") {
        state.advance(";").finish(true)
    }
    else {
        state.finish(false)
    }
}

/// `when`
#[inline]
pub fn parse_when(input: ParseState) -> ParseResult<&str> {
    if input.residual.starts_with("if") {
        input.advance_view("if".len())
    }
    else if input.residual.starts_with("when") {
        input.advance_view("when".len())
    }
    else {
        StopBecause::missing_string("when", input.start_offset)?
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

#[inline]
pub fn parse_comma(input: ParseState) -> ParseResult<&str> {
    input.match_str(",")
}

#[inline]
pub fn parse_in(input: ParseState) -> ParseResult<&str> {
    if input.residual.starts_with("in") {
        input.advance_view("in".len())
    }
    else if input.residual.starts_with('∈') {
        input.advance_view("∈".len())
    }
    else {
        StopBecause::missing_character('∈', input.start_offset)?
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
pub fn parse_any_name_path<F>(input: ParseState, id: F) -> ParseResult<NamePathNode>
where
    F: Fn(ParseState) -> ParseResult<IdentifierNode> + Copy,
{
    let (state, head) = id(input)?;
    let (state, rest) = state.match_repeats(|s| {
        let (state, _) = s.skip(ignore).match_fn(parse_name_join_dot)?;
        state.skip(ignore).match_fn(id)
    })?;
    let mut names = Vec::with_capacity(rest.len() + 1);
    names.push(head);
    names.extend(rest);
    state.finish(NamePathNode { names, span: get_span(input, state) })
}
