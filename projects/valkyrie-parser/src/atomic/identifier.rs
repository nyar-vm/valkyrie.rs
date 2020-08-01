use super::*;
use valkyrie_ast::PrettyPrint;

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
        let id = IdentifierNode::new(m.as_str(), m.range());
        state.finish(id)
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::Function(self.pretty_string(144))
    }
}
