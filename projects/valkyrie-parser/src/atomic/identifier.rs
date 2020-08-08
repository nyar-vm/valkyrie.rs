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
        let range = m.range();
        let id = IdentifierNode::new(m.as_str(), range.start as u32, range.end as u32);
        state.finish(id)
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::Function(self.pretty_string(144))
    }
}
