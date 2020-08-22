use super::*;
use valkyrie_ast::LambdaSlotNode;

pub static IDENTIFIER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?x)(
      [∞∅]
    | (?P<regular>(?:\p{XID_Start}|_)\p{XID_Continue}*)
    | `(?P<escaped>(?:\\.|[^`])*)`
)",
    )
    .unwrap()
});

impl ThisParser for IdentifierNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&IDENTIFIER, "IDENTIFIER")?;
        let start = input.start_offset as u32;
        let end = (input.start_offset + m.range().end) as u32;
        let id = IdentifierNode::new(m.as_str(), start..end);
        state.finish(id)
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::Function(self.pretty_string(144))
    }
}

impl ThisParser for LambdaSlotNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_char('$')?;
        let (state, id) = state.match_fn(IdentifierNode::parse)?;
        state.finish(LambdaSlotNode::new(id.name, get_span(input, state)))
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::keyword(self.name.as_str())
    }
}
