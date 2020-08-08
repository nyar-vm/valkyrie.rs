use super::*;

impl ThisParser for OperatorNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

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

static INFIX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)(
      [.]{2}[=<]
    # | [.]{1,3}
    # | [⟦⟧⟬⟭{}\[\]()]
    | [$§¶^]
    | @[*!?@]?
    # contains
    | (\b(not)\b\s+)?\b(in)\b
    | [∈∊∉]
    # is
    | \b(is)\b(\s+\b(not)\b)?
    | [⊑⋢]
    # start with <, >
    | === | ≡ 
    | !== | =!= | ≢
    | == | ≖
    | != | ≠
    | <=> 
    # start with <, >
    | >{1,3} | >= | /> | ≥ | ⩾ | ≫ | ⋙
    | <{1,3} | <= | </ | ≤ | ⩽ | ≪ | ⋘ |  <: | <!
    # start with :
    | :> | := | ≔
    # start with -
    | -= | -> | ⟶ | -{1,2}
    # start with ~
    | ~> | ~
    # start with +
    | [+]= | [+]> | [+]{1,2}
    # start with *
    | [*]=?
    # start with / or % or ÷
    | /=?
    | ÷=?
    | %=?
    # start with &
    | &> | &{1,2} | ≻
    | [|]> | [|]{1,2} | ⊁
    | ⊻=? | ⊼=? | ⊽=? | [⩕⩖]
    # start with ?
    | [?]{3}
    # start with =
    | => | ⇒
    | =
    # unicode
    | [⨳∀∁∂∃∄¬±√∛∜⊹⋗]
    | [↻↺⇆↹⇄⇋⇌⇅]
)"#,
    )
    .unwrap()
});

static TYPE_INFIX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)(
      [&|]
    | [+]
    | [⊑⋢]
)"#,
    )
    .unwrap()
});

impl ValkyrieInfix {
    pub fn parse(input: ParseState, type_level: bool) -> ParseResult<Self> {
        let (state, m) = match type_level {
            true => input.match_regex(&TYPE_INFIX, "TYPE_INFIX"),
            false => input.match_regex(&INFIX, "INFIX"),
        }?;
        let id = ValkyrieInfix::new(m.as_str(), get_span(input, state));
        state.finish(id)
    }
}

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
