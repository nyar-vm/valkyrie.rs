use super::*;
use valkyrie_ast::PrettyPrint;

impl ThisParser for NamePathNode {
    /// `id (~ :: ~ b)*`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let mut names = Vec::new();
        let (state, id) = input.match_fn(IdentifierNode::parse)?;
        names.push(id);
        let (state, _) = state.match_repeats(|s| pare_colon_id(s, &mut names))?;
        state.finish(NamePathNode::new(names, state.away_from(input)))
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::Function(self.pretty_string(144))
    }
}

fn pare_colon_id<'i>(input: ParseState<'i>, names: &mut Vec<IdentifierNode>) -> ParseResult<'i, ()> {
    let (state, _) = parse_name_join(input)?;
    let (state, id) = state.match_fn(|s| IdentifierNode::parse(s))?;
    names.push(id);
    state.finish(())
}
