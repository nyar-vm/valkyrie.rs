use super::*;
use valkyrie_ast::{AnnotationKind, AnnotationList, AnnotationNode};

impl ThisParser for AnnotationKind {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .or_else(|s| s.match_str("@@").map_inner(|_| AnnotationKind::Environment))
            .or_else(|s| s.match_str("@!").map_inner(|_| AnnotationKind::NonCapture))
            .or_else(|s| s.match_str("@").map_inner(|_| AnnotationKind::Normal))
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for AnnotationNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, kind) = AnnotationKind::parse(input)?;
        let (state, name) = Identifier::parse(state)?;

        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for AnnotationList {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, kind) = AnnotationKind::parse(input)?;
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}
