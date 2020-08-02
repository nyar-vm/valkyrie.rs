use super::*;

impl Debug for ValkyrieInfix {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Infix({}, {:?})", self.as_operator().kind.as_str(), self.range)
    }
}

impl Debug for ValkyriePrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Prefix({}, {:?})", self.normalized, self.range)
    }
}

impl Debug for ValkyrieSuffix {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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
            "++" => ValkyrieOperator::Concat,
            "+" => ValkyrieOperator::Plus,
            "-" => ValkyrieOperator::Minus,
            "*" => ValkyrieOperator::Multiply,
            "/" => ValkyrieOperator::Divide,
            "^" => ValkyrieOperator::Power,
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
            "!" => ValkyrieOperator::Unwrap,
            "?" => ValkyrieOperator::Raise,
            "℃" => ValkyrieOperator::Celsius,
            "℉" => ValkyrieOperator::Fahrenheit,
            "%" => ValkyrieOperator::DivideByDecimalPower(2),
            "‰" => ValkyrieOperator::DivideByDecimalPower(3),
            "‱" => ValkyrieOperator::DivideByDecimalPower(4),
            _ => unreachable!("Unknown operator: {}", self.normalized),
        };
        OperatorNode::new(kind, self.range.clone())
    }
}
