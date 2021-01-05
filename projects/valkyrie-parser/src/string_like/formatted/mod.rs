use crate::ThisParser;
use lispify::Lisp;
use pex::{ParseResult, ParseState};
use valkyrie_ast::ExpressionFormatted;

impl ThisParser for ExpressionFormatted {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn lispify(&self) -> Lisp {
        todo!()
    }
}
