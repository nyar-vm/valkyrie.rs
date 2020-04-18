mod escaper;
pub use self::escaper::StringRewrite;
use pex::{ParseResult, ParseState};
use pex::helpers::{comment_line, whitespace};

/// Ignores whitespace and comments.
#[inline]
pub fn ignore(input: ParseState) -> ParseResult<()> {
    input.begin_choice()
        .or_else(|s| whitespace(s).map_inner(|_| ()))
        .or_else(|s| comment_line(s, "//").map_inner(|_| ()))
        .end_choice()
}