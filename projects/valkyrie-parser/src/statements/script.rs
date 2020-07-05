use super::*;

pub struct ScriptRoot {
    pub wrap: Vec<StatementNode>,
}

impl ThisParser for ScriptRoot {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}
