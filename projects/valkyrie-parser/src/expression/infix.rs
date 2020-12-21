use super::*;

impl Debug for ValkyrieInfix {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Infix({}, {:?})", self.as_operator().kind.as_str(), self.span)
    }
}

impl ThisParser for MonadicCall {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, mark) = match input.match_char('?') {
            ParseResult::Pending(s, _) => s.finish(true),
            ParseResult::Stop(_) => input.finish(false),
        }?;
        let state = state.skip(ignore);
        state.finish(Self { mark })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl ThisParser for MonadicDotCall {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, mark) = match input.match_char('?') {
            ParseResult::Pending(s, _) => s.finish(true),
            ParseResult::Stop(_) => input.finish(false),
        }?;
        let (state, _) = char('.')(state.skip(ignore))?;
        let state = state.skip(ignore);
        state.finish(Self { mark })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

static INFIX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?x)(
      [.]{2}[=<]
    | [‥…]
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
    | :>
    # start with -
    | -> | ⟶ | -{1,2}=?
    # start with ~
    | ~> | ~
    # start with +
    | [+]> | [+]{1,2}=?
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

impl ValkyrieInfix {
    pub fn new<S: AsRef<str>>(infix: S, range: Range<u32>) -> ValkyrieInfix {
        let text = infix.as_ref();
        let mut normalized = String::with_capacity(text.len());
        for c in text.chars() {
            match c {
                ' ' => continue,
                '∈' | '∊' => normalized.push_str("in"),
                '∉' => normalized.push_str("notin"),
                '⊑' => normalized.push_str("is"),
                '⋢' => normalized.push_str("isnot"),
                '≖' => normalized.push_str("=="),
                '≠' => normalized.push_str("!="),
                '≡' => normalized.push_str("==="),
                '≢' => normalized.push_str("=!="),
                '≫' => normalized.push_str(">>"),
                '≪' => normalized.push_str("<<"),
                '⋙' => normalized.push_str(">>>"),
                '⋘' => normalized.push_str("<<<"),
                _ => normalized.push(c),
            }
        }
        ValkyrieInfix { normalized, span: range }
    }
    pub fn precedence(&self) -> Precedence {
        Precedence(self.as_operator().kind.precedence())
    }
    pub fn associativity(&self) -> Associativity {
        match self.normalized.as_str() {
            "^" => Associativity::Right,
            _ => Associativity::Left,
        }
    }
    pub fn as_operator(&self) -> OperatorNode {
        let kind = match self.normalized.as_str() {
            "++" => ValkyrieOperator::Concat,
            "+" => ValkyrieOperator::Plus,
            "-" => ValkyrieOperator::Minus,
            "*" => ValkyrieOperator::Multiply,
            "/" => ValkyrieOperator::Divide,
            "^" => ValkyrieOperator::Power,
            "|" => ValkyrieOperator::BitOr,
            "&" => ValkyrieOperator::BitAnd,
            ">" => ValkyrieOperator::Greater,
            ">>" => ValkyrieOperator::MuchGreater,
            ">>>" => ValkyrieOperator::VeryMuchGreater,
            "<" => ValkyrieOperator::Less,
            "<<" => ValkyrieOperator::MuchLess,
            "<<<" => ValkyrieOperator::VeryMuchLess,
            "==" => ValkyrieOperator::Equal(true),
            "!=" => ValkyrieOperator::Equal(false),
            "===" => ValkyrieOperator::StrictlyEqual(true),
            "!==" | "=!=" => ValkyrieOperator::StrictlyEqual(false),
            "in" => ValkyrieOperator::Belongs(true),
            "notin" => ValkyrieOperator::Belongs(false),
            "is" => ValkyrieOperator::IsA(true),
            "isnot" => ValkyrieOperator::IsA(false),
            "+=" => ValkyrieOperator::PlusAssign,
            "=" => ValkyrieOperator::Assign,
            "-=" => ValkyrieOperator::MinusAssign,
            _ => unreachable!("Unknown operator: {}", self.normalized),
        };
        OperatorNode::new(kind, self.span.clone())
    }
}
