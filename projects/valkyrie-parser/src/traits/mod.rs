use crate::helpers::ignore;
use lispify::Lisp;
use pex::{ParseResult, ParseState};
use valkyrie_error::{ValkyrieError, ValkyrieResult};

pub trait ThisParser
where
    Self: Sized,
{
    fn parse(input: ParseState) -> ParseResult<Self>;
    fn parse_text(input: &str) -> ValkyrieResult<Self> {
        let input = ParseState::new(input);
        let (state, repl) = match Self::parse(input.skip(ignore)) {
            ParseResult::Pending(s, v) => (s.skip(ignore), v),
            ParseResult::Stop(e) => Err(ValkyrieError::custom(format!("Failed to parse REPL: {:?}", e)))?,
        };
        if !state.residual.is_empty() {
            Err(ValkyrieError::custom(format!("Expect EOF, found:\n{}", state.residual)))?
        }
        Ok(repl)
    }
    fn as_lisp(&self) -> Lisp;
}
