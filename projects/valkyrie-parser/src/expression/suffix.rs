use super::*;
use fancy_regex::Regex;
impl Debug for ValkyrieSuffix {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Postfix({}, {:?})", self.normalized, self.span)
    }
}

static POSTFIX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)(
      ↑
    # Quick raise
    | !(?!=)
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
        let result = match POSTFIX.find_from_pos(&input.residual, 0) {
            Ok(Some(s)) => s,
            _ => StopBecause::missing_string("PREFIX", input.start_offset)?,
        };
        let state = input.advance(result.end());
        state.finish(ValkyrieSuffix { normalized: result.as_str().to_string(), span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl ValkyrieSuffix {
    // pub fn new<S: ToString>(s: S, range: Range<u32>) -> ValkyrieSuffix {
    //     ValkyrieSuffix { normalized: s.to_string(), span: range }
    // }
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
