use valkyrie_ast::ModifierPart;
use super::*;

impl ThisParser for LetBindNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (input, _) = input.match_str("let")?;
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

