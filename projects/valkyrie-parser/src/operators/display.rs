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
    pub fn as_operator(&self) -> OperatorNode {
        let kind = match self.normalized.as_str() {
            "+" => OperatorKind::Positive,
            "-" => OperatorKind::Negative,
            "*" => OperatorKind::Unbox,
            "⁑" | "**" => OperatorKind::Unpack,
            "⁂" | "***" => OperatorKind::UnpackAll,
            "!" => OperatorKind::Not,
            "⅟" => OperatorKind::Minus,
            "√" => OperatorKind::Surd(2),
            "∛" => OperatorKind::Surd(3),
            "∜" => OperatorKind::Surd(4),
            _ => unreachable!("Unknown operator: {}", self.normalized),
        };
        OperatorNode::new(kind, self.range.clone())
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
    pub fn as_operator(&self) -> OperatorNode {
        let kind = match self.normalized.as_str() {
            "++" => OperatorKind::Concat,
            "+" => OperatorKind::Plus,
            "-" => OperatorKind::Minus,
            "*" => OperatorKind::Multiply,
            "/" => OperatorKind::Divide,
            "^" => OperatorKind::Power,
            ">" => OperatorKind::Greater,
            ">>" => OperatorKind::MuchGreater,
            ">>>" => OperatorKind::VeryMuchGreater,
            "<" => OperatorKind::Less,
            "<<" => OperatorKind::MuchLess,
            "<<<" => OperatorKind::VeryMuchLess,
            "==" => OperatorKind::Equal(true),
            "!=" => OperatorKind::Equal(false),
            "===" => OperatorKind::StrictlyEqual(true),
            "!==" | "=!=" => OperatorKind::StrictlyEqual(false),
            "in" => OperatorKind::Belongs(true),
            "notin" => OperatorKind::Belongs(false),
            "is" => OperatorKind::IsA(true),
            "isnot" => OperatorKind::IsA(false),
            _ => unreachable!("Unknown operator: {}", self.normalized),
        };
        OperatorNode::new(kind, self.range.clone())
    }
}

impl ValkyrieSuffix {
    pub fn new<S: ToString>(s: S, range: Range<usize>) -> ValkyrieSuffix {
        ValkyrieSuffix { normalized: s.to_string(), range }
    }
    pub fn precedence(&self) -> Precedence {
        Precedence(self.as_operator().kind.precedence())
    }
    pub fn as_operator(&self) -> OperatorNode {
        let kind = match self.normalized.as_str() {
            "!" => OperatorKind::Unwrap,
            "?" => OperatorKind::Raise,
            "℃" => OperatorKind::Celsius,
            "℉" => OperatorKind::Fahrenheit,
            "%" => OperatorKind::DivideByDecimalPower(2),
            "‰" => OperatorKind::DivideByDecimalPower(3),
            "‱" => OperatorKind::DivideByDecimalPower(4),
            _ => unreachable!("Unknown operator: {}", self.normalized),
        };
        OperatorNode::new(kind, self.range.clone())
    }
}
