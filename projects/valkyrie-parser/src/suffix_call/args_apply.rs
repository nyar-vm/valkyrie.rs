use super::*;
use crate::expression::TypingExpression;
use valkyrie_ast::{ApplyArgumentNode, ApplyArgumentTerm, ArgumentKeyNode, ExpressionNode};

impl ThisParser for ApplyArgumentTerm {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, term) = ArgumentTermNode::<ArgumentKeyNode, TypingExpression, ExpressionNode>::parse(input)?;
        state.finish(ApplyArgumentTerm { term: term.map_value(|v| v.wrapper) })
    }

    fn as_lisp(&self) -> Lisp {
        self.term.as_lisp()
    }
}
