mod escaper;

pub use self::escaper::StringRewrite;

use std::sync::LazyLock;
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
