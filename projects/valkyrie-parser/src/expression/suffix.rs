use super::*;

static POSTFIX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)(
      [?]
    # Temperature
    | [℃℉]
    # Percents
    | [%‰‱]
    # Transpose,adjoint, conjugate
    | [ᵀᴴ]
)"#,
    )
    .unwrap()
});

impl ValkyrieSuffix {
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&POSTFIX, "POSTFIX")?;
        let id = ValkyrieSuffix { normalized: m.as_str().to_string(), span: get_span(input, state) };
        state.finish(id)
    }
}
