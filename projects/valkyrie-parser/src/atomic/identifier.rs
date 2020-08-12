use super::*;

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
