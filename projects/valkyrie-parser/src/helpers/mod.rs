mod escaper;

pub use self::escaper::StringRewrite;
use crate::{expression::ValkyrieExpression, traits::ThisParser};
use pex::{ParseResult, ParseState};
use regex::Regex;
use std::sync::LazyLock;
use valkyrie_ast::{NamepathNode, NumberNode};

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
        .or_else(|s| NamepathNode::parse(s).map_inner(Into::into))
        .or_else(|s| NumberNode::parse(s).map_inner(Into::into))
        .or_else(|s| StringLiteralNode::parse(s).map_inner(Into::into))
        .end_choice()
}
