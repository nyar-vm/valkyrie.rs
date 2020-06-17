mod escaper;

pub use self::escaper::StringRewrite;
use crate::{expression::ValkyrieExpression, traits::ThisParser};
use std::sync::LazyLock;
use valkyrie_ast::{NamePathNode, NumberLiteralNode, StringLiteralNode, TableNode};
use valkyrie_types::third_party::pex::{ParseResult, ParseState, Regex, StopBecause};

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
        .or_else(|s| NamePathNode::parse(s).map_inner(Into::into))
        .or_else(|s| NumberLiteralNode::parse(s).map_inner(Into::into))
        .or_else(|s| StringLiteralNode::parse(s).map_inner(Into::into))
        .or_else(|s| TableNode::parse(s).map_inner(Into::into))
        .end_choice()
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
