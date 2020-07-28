use super::*;

impl ValkyrieOperator {
    /// Get the normalised string representation of the operator.
    pub fn as_str(&self) -> &'static str {
        match self {
            ValkyrieOperator::Not => "!",
            ValkyrieOperator::Concat => "++",
            ValkyrieOperator::Positive => "+",
            ValkyrieOperator::Negative => "-",
            ValkyrieOperator::Plus => "+",
            ValkyrieOperator::Minus => "-",
            ValkyrieOperator::Multiply => "*",
            ValkyrieOperator::Divide => "/",
            ValkyrieOperator::Power => "^",
            ValkyrieOperator::Unwrap => "!",
            ValkyrieOperator::Raise => "?",
            ValkyrieOperator::Celsius => "℃",
            ValkyrieOperator::Fahrenheit => "℉",
            ValkyrieOperator::Transpose => "ᵀ",
            ValkyrieOperator::Transjugate => "ᴴ",
            ValkyrieOperator::Hermitian => "Hermitian",
            ValkyrieOperator::Unbox => "*",
            ValkyrieOperator::Unpack => "⁑",
            ValkyrieOperator::UnpackAll => "⁂",
            ValkyrieOperator::Greater => ">",
            ValkyrieOperator::MuchGreater => "≫",
            ValkyrieOperator::VeryMuchGreater => "⋙",
            ValkyrieOperator::Less => "<",
            ValkyrieOperator::MuchLess => "≪",
            ValkyrieOperator::VeryMuchLess => "⋘",
            ValkyrieOperator::Belongs(v) => match v {
                true => "∈",
                false => "∉",
            },
            ValkyrieOperator::IsA(v) => match v {
                true => "⊑",
                false => "⋢",
            },
            ValkyrieOperator::Equal(v) => match v {
                true => "≖",
                false => "≠",
            },
            ValkyrieOperator::StrictlyEqual(v) => match v {
                true => "≡",
                false => "≢",
            },
            ValkyrieOperator::Inverse => "i",
            ValkyrieOperator::Surd(v) => match v {
                3 => "∛",
                4 => "∜",
                _ => "√",
            },
            ValkyrieOperator::DivideByDecimalPower(v) => match v {
                3 => "‰",
                4 => "‱",
                _ => "%",
            },
        }
    }
}

impl PrettyPrint for OperatorNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> RefDoc<'a, ColorSpec> {
        allocator.text(self.kind.as_str()).annotate(allocator.number_style()).into_doc()
    }
}

impl IndentDisplay for ValkyrieOperator {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl IndentDisplay for OperatorNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        self.kind.indent_fmt(f)
    }
}

impl Display for ValkyrieOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

impl Display for OperatorNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.kind.as_str())
    }
}

impl<E: IndentDisplay> IndentDisplay for PrefixNode<E> {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        self.operator.indent_fmt(f)?;
        self.body.indent_fmt(f)
    }
}

impl<E: IndentDisplay> IndentDisplay for InfixNode<E> {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        self.lhs.indent_fmt(f)?;
        f.write_char(' ')?;
        self.operator.indent_fmt(f)?;
        f.write_char(' ')?;
        self.rhs.indent_fmt(f)
    }
}

impl<E: IndentDisplay> IndentDisplay for PostfixNode<E> {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        self.body.indent_fmt(f)?;
        self.operator.indent_fmt(f)
    }
}

impl<E: IndentDisplay> Display for PrefixNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

impl<E: IndentDisplay> Display for InfixNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

impl<E: IndentDisplay> Display for PostfixNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}
