use super::*;

impl Debug for ValkyrieSuffix {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Postfix({}, {:?})", self.normalized, self.span)
    }
}

static POSTFIX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)(
      ↑
    # Factorial
    # | [!]
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

impl ThisParser for ValkyrieSuffix {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&POSTFIX, "POSTFIX")?;
        let id = ValkyrieSuffix { normalized: m.as_str().to_string(), span: get_span(input, state) };
        state.finish(id)
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl ValkyrieSuffix {
    pub fn new<S: ToString>(s: S, range: Range<u32>) -> ValkyrieSuffix {
        ValkyrieSuffix { normalized: s.to_string(), span: range }
    }
    pub fn precedence(&self) -> Precedence {
        Precedence(self.as_operator().kind.precedence())
    }
    pub fn as_operator(&self) -> OperatorNode {
        let kind = match self.normalized.as_str() {
            "!" => ValkyrieOperator::QuickRaise,
            // "?" => ValkyrieOperator::Raise,
            "℃" => ValkyrieOperator::Celsius,
            "℉" => ValkyrieOperator::Fahrenheit,
            "%" => ValkyrieOperator::DivideByDecimalPower(2),
            "‰" => ValkyrieOperator::DivideByDecimalPower(3),
            "‱" => ValkyrieOperator::DivideByDecimalPower(4),
            _ => unreachable!("Unknown operator: {}", self.normalized),
        };
        OperatorNode::new(kind, self.span.clone())
    }
}
