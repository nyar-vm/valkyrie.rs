use super::*

static PREFIX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)(
      [+\-±]
    | [¬!~]
    | [⅟√∛∜]
    | [*]{1,3}
    | [⁑⁂]
)"#,
    )
        .unwrap()
});

impl ValkyriePrefix {
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&PREFIX, "PREFIX")?;
        let id = ValkyriePrefix { normalized: m.as_str().to_string(), span: get_span(input, state) };
        state.finish(id)
    }
}