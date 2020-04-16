use std::cell::LazyCell;
use std::sync::LazyLock;
use regex_automata::dfa::regex::Regex;
use super::*;


impl FromStr for ValkyrieIdentifier {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, Self::parse)
    }
}

pub static IDENTIFIER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?ux)
    ^((?:\p{XID_Start}|_)\p{XID_Continue}*)
|   ^`(\\.|[^`])*`
    ").unwrap()
});

impl ValkyrieIdentifier {
    /// - regular: `\p{XID_Start}|_)\p{XID_Continue}*`
    /// - escaped: ``` `(\.|[^`])*` ```
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }
}

#[test]
pub fn test() {
    let input = "`_hello` world";
    for term in IDENTIFIER.find_leftmost_iter(input.as_bytes()) {
        println!("{:?}", term);
    }
}
