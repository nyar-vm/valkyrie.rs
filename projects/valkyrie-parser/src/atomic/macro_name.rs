use super::*;

impl ThisParser for MacroPathNode {
    /// `a::b::c.d.e.f`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, path) = input.match_fn(NamePathNode::parse)?;
        let (state, names) = state.match_repeats(pare_dot_id)?;
        state.finish(MacroPathNode::new(path, names, get_span(input, state)))
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

/// ~ . ~ id
fn pare_dot_id(input: ParseState) -> ParseResult<IdentifierNode> {
    let (state, _) = input.skip(ignore).match_char('.')?;
    let (state, id) = state.skip(ignore).match_fn(IdentifierNode::parse)?;
    state.finish(id)
}