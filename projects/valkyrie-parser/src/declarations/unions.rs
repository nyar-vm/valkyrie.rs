use super::*;

impl ThisParser for UnionDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for VariantDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, name) = IdentifierNode::parse(input)?;
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for FieldDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, name) = IdentifierNode::parse(input)?;
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for ModifiersNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}
