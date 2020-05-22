use super::*;

static INFIX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)(
      [.]{2}[=<]
    # | [.]{1,3}
    # | [⟦⟧⁅⁆⟬⟭{}\[\]()]
    | [$§¶^]
    | @[*!?@]?
    # contains
    | (\b(not)\b\s+)?\b(in)\b
    | [∈∊∉]
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
    | [?]{3} | [?]
    # start with =
    | => | ⇒
    | =
    # unicode
    | [⊑⋢⨳∀∁∂∃∄¬±√∛∜⊹⋗]
    | [↻↺⇆↹⇄⇋⇌⇅]
)"#,
    )
    .unwrap()
});

impl FromStr for ValkyrieInfix {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, StopBecause> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieInfix::parse)
    }
}

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

static PREFIX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)(
      [+\-±]
    | [¬~]
    | [⅟√∛∜]
    # | [*]{1,2}
)"#,
    )
    .unwrap()
});

impl FromStr for ValkyriePrefix {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, StopBecause> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyriePrefix::parse)
    }
}

impl ValkyriePrefix {
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&PREFIX, "PREFIX")?;
        let id = ValkyriePrefix { normalized: m.as_str().to_string(), range: state.away_from(input) };
        state.finish(id)
    }
}

impl Debug for ValkyriePrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Infix({}, {:?})", self.normalized, self.range)
    }
}

impl ValkyriePrefix {
    pub fn new<S: ToString>(s: S, range: Range<usize>) -> ValkyriePrefix {
        ValkyriePrefix { normalized: s.to_string(), range }
    }
    pub fn precedence(&self) -> Precedence {
        self.as_operator().kind.precedence()
    }
    pub fn as_operator(&self) -> ValkyrieOperator {
        let kind = match self.normalized.as_str() {
            "+" => ValkyrieOperatorKind::Positive,
            "-" => ValkyrieOperatorKind::Negative,
            "*" => ValkyrieOperatorKind::Unbox,
            "**" => ValkyrieOperatorKind::Unpack,
            "!" => ValkyrieOperatorKind::Not,
            "⅟" => ValkyrieOperatorKind::Minus,
            "√" => ValkyrieOperatorKind::Surd(2),
            "∛" => ValkyrieOperatorKind::Surd(3),
            "∜" => ValkyrieOperatorKind::Surd(4),
            _ => unreachable!("Unknown operator: {}", self.normalized),
        };
        ValkyrieOperator::new(kind, self.range.clone())
    }
}

impl Debug for ValkyrieSuffix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Infix({}, {:?})", self.normalized, self.range)
    }
}

impl ValkyrieSuffix {
    pub fn new<S: ToString>(s: S, range: Range<usize>) -> ValkyrieSuffix {
        ValkyrieSuffix { normalized: s.to_string(), range }
    }
    pub fn precedence(&self) -> Precedence {
        self.as_operator().kind.precedence()
    }
    pub fn as_operator(&self) -> ValkyrieOperator {
        let kind = match self.normalized.as_str() {
            "!" => ValkyrieOperatorKind::Unwrap,
            "?" => ValkyrieOperatorKind::Raise,
            "℃" => ValkyrieOperatorKind::Celsius,
            "℉" => ValkyrieOperatorKind::Fahrenheit,
            "%" => ValkyrieOperatorKind::DivideByDecimalPower(2),
            "‰" => ValkyrieOperatorKind::DivideByDecimalPower(3),
            "‱" => ValkyrieOperatorKind::DivideByDecimalPower(4),
            _ => unreachable!("Unknown operator: {}", self.normalized),
        };
        ValkyrieOperator::new(kind, self.range.clone())
    }
}

impl FromStr for ValkyrieSuffix {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, StopBecause> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieSuffix::parse)
    }
}

impl ValkyrieSuffix {
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&POSTFIX, "POSTFIX")?;
        let id = ValkyrieSuffix { normalized: m.as_str().to_string(), range: state.away_from(input) };
        state.finish(id)
    }
}
