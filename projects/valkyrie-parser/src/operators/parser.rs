use super::*;

impl ThisParser for OperatorNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl FromStr for ValkyrieInfix {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, StopBecause> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieInfix::parse)
    }
}
impl FromStr for ValkyriePrefix {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, StopBecause> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyriePrefix::parse)
    }
}
impl FromStr for ValkyrieSuffix {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, StopBecause> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieSuffix::parse)
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
        let id = ValkyriePrefix { normalized: m.as_str().to_string(), range: state.away_from(input) };
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

impl ValkyrieInfix {
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&INFIX, "INFIX")?;
        let id = ValkyrieInfix::new(m.as_str(), state.away_from(input));
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
        let id = ValkyrieSuffix { normalized: m.as_str().to_string(), range: state.away_from(input) };
        state.finish(id)
    }
}
