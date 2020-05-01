mod escaper;
pub use self::escaper::StringRewrite;
use crate::{expression::ValkyrieExpression, number::ValkyrieNumber, symbol::ValkyrieNamepath};
use pex::{
    helpers::{comment_line, whitespace},
    ParseResult, ParseState,
};

/// Ignores whitespace and comments.
#[inline]
pub fn ignore(input: ParseState) -> ParseResult<()> {
    input
        .begin_choice()
        .or_else(|s| whitespace(s).map_inner(|_| ()))
        .or_else(|s| comment_line(s, "//").map_inner(|_| ()))
        .end_choice()
}

#[inline]
pub fn parse_value(input: ParseState) -> ParseResult<ValkyrieExpression> {
    input
        .begin_choice()
        .or_else(|s| ValkyrieNumber::parse(s).map_inner(Into::into))
        .or_else(|s| ValkyrieNamepath::parse(s).map_inner(|s| ValkyrieExpression::Symbol(Box::new(s))))
        .end_choice()
}
