use super::*;

impl Debug for ValkyriePrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Prefix({}, {:?})", self.normalized, self.span)
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

impl ThisParser for ValkyriePrefix {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&PREFIX, "PREFIX")?;
        let id = ValkyriePrefix { normalized: m.as_str().to_string(), span: get_span(input, state) };
        state.finish(id)
    }

    fn lispify(&self) -> Lisp {
        unreachable!()
    }
}

impl ValkyriePrefix {
    // pub fn new<S: ToString>(s: S, range: Range<u32>) -> ValkyriePrefix {
    //     ValkyriePrefix { normalized: s.to_string(), span: range }
    // }
    pub fn precedence(&self) -> Precedence {
        Precedence(self.as_operator().kind.precedence())
    }
    pub fn as_operator(&self) -> OperatorNode {
        let kind = match self.normalized.as_str() {
            "+" => ValkyrieOperator::Positive,
            "-" => ValkyrieOperator::Negative,
            "*" => ValkyrieOperator::Unbox,
            "⁑" | "**" => ValkyrieOperator::Unpack,
            "⁂" | "***" => ValkyrieOperator::UnpackAll,
            "!" => ValkyrieOperator::Not,
            "⅟" => ValkyrieOperator::Minus,
            "√" => ValkyrieOperator::Surd(2),
            "∛" => ValkyrieOperator::Surd(3),
            "∜" => ValkyrieOperator::Surd(4),
            _ => unreachable!("Unknown operator: {}", self.normalized),
        };
        OperatorNode::new(kind, self.span.clone())
    }
}
