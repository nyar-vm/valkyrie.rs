use super::*;

pub static TEMPLATE_TEXT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?x)(
      (?!)(\{[{%#]).
)",
    )
    .unwrap()
});

impl ThisParser for StringTemplateNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}
