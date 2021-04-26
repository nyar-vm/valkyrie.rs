use super::*;

impl ThisParser for ApplyArgumentTerm {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, term) = ArgumentTermNode::<ArgumentKeyNode, TypingExpression, ExpressionNode>::parse(input)?;
        state.finish(ApplyArgumentTerm { term: term.map_value(|v| v.as_normal()) })
    }
}
