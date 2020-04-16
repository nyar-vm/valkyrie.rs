use pex::helpers::{make_from_str, whitespace};
use super::*;



impl FromStr for ValkyrieIdentifier {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, Self::parse)
    }
}

impl ValkyrieIdentifier {
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }
}

