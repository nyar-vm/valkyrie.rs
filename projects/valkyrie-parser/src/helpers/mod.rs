mod escaper;

pub use self::escaper::StringRewrite;
use crate::{expression::ValkyrieExpression, number::ValkyrieNumber, string::ValkyrieString, symbol::ValkyrieNamepath};
use pex::{ParseResult, ParseState};
use regex::Regex;
use std::sync::LazyLock;

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
pub fn ignore(input: ParseState) -> ParseResult<()> {
    input.match_regex(&IGNORE, "IGNORE").map_inner(|_| ())
}

#[inline]
pub fn parse_value(input: ParseState) -> ParseResult<ValkyrieExpression> {
    input
        .begin_choice()
        .or_else(|s| ValkyrieNamepath::parse(s).map_inner(Into::into))
        .or_else(|s| ValkyrieNumber::parse(s).map_inner(Into::into))
        .or_else(|s| ValkyrieString::parse(s).map_inner(Into::into))
        .end_choice()
}
