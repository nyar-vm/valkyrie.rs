use pex::{ParseResult, ParseState};
use pex::helpers::{comment_line, whitespace};

pub fn ignore(input: ParseState) -> ParseResult<()> {
    input.begin_choice()
        .or_else(whitespace)
        .or_else(|s| comment_line(s, "//").map_inner(|_| ()))
        .parse()
}