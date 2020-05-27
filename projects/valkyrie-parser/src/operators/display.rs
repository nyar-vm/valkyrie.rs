use super::*;

impl Debug for ValkyrieInfix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Infix({}, {:?})", self.as_operator(), self.range)
    }
}

impl Debug for ValkyriePrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Prefix({}, {:?})", self.normalized, self.range)
    }
}

impl Debug for ValkyrieSuffix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Postfix({}, {:?})", self.normalized, self.range)
    }
}

impl ValkyriePrefix {
    pub fn new<S: ToString>(s: S, range: Range<usize>) -> ValkyriePrefix {
        ValkyriePrefix { normalized: s.to_string(), range }
    }
    pub fn precedence(&self) -> Precedence {
        Precedence(self.as_operator().kind.precedence())
    }
    pub fn as_operator(&self) -> ValkyrieOperator {
        let kind = match self.normalized.as_str() {
            "+" => ValkyrieOperatorKind::Positive,
            "-" => ValkyrieOperatorKind::Negative,
            "*" => ValkyrieOperatorKind::Unbox,
            "⁑" | "**" => ValkyrieOperatorKind::Unpack,
            "⁂" | "***" => ValkyrieOperatorKind::UnpackAll,
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

impl ValkyrieInfix {
    pub fn new<S: AsRef<str>>(infix: S, range: Range<usize>) -> ValkyrieInfix {
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
        ValkyrieInfix { normalized, range }
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
    pub fn as_operator(&self) -> ValkyrieOperator {
        let kind = match self.normalized.as_str() {
            "++" => ValkyrieOperatorKind::Concat,
            "+" => ValkyrieOperatorKind::Plus,
            "-" => ValkyrieOperatorKind::Minus,
            "*" => ValkyrieOperatorKind::Multiply,
            "/" => ValkyrieOperatorKind::Divide,
            "^" => ValkyrieOperatorKind::Power,
            ">" => ValkyrieOperatorKind::Greater,
            ">>" => ValkyrieOperatorKind::MuchGreater,
            ">>>" => ValkyrieOperatorKind::VeryMuchGreater,
            "<" => ValkyrieOperatorKind::Less,
            "<<" => ValkyrieOperatorKind::MuchLess,
            "<<<" => ValkyrieOperatorKind::VeryMuchLess,
            "==" => ValkyrieOperatorKind::Equal(true),
            "!=" => ValkyrieOperatorKind::Equal(false),
            "===" => ValkyrieOperatorKind::StrictlyEqual(true),
            "!==" | "=!=" => ValkyrieOperatorKind::StrictlyEqual(false),
            "in" => ValkyrieOperatorKind::Belongs(true),
            "notin" => ValkyrieOperatorKind::Belongs(false),
            "is" => ValkyrieOperatorKind::IsA(true),
            "isnot" => ValkyrieOperatorKind::IsA(false),
            _ => unreachable!("Unknown operator: {}", self.normalized),
        };
        ValkyrieOperator::new(kind, self.range.clone())
    }
}

impl ValkyrieSuffix {
    pub fn new<S: ToString>(s: S, range: Range<usize>) -> ValkyrieSuffix {
        ValkyrieSuffix { normalized: s.to_string(), range }
    }
    pub fn precedence(&self) -> Precedence {
        Precedence(self.as_operator().kind.precedence())
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
