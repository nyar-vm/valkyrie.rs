use super::*;

pub static TEMPLATE_TEXT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?x)(
      ((?!(\{[{%#])).)+
)",
    )
    .unwrap()
});

impl ThisParser for StringTemplateNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, text) = parse_text(input)?;
        state.finish(StringTemplateNode { items: vec![text], span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

fn parse_text(input: ParseState) -> ParseResult<StatementNode> {
    let (state, text) = input.match_regex(&TEMPLATE_TEXT, "TEMPLATE_TEXT")?;
    state.finish(StatementNode::string(text.as_str(), get_span(input, state)))
}
