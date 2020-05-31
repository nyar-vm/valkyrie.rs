use super::*;

impl Display for OperatorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorKind::Not => f.write_char('!'),
            OperatorKind::Concat => f.write_str("++"),
            OperatorKind::Positive => f.write_char('+'),
            OperatorKind::Negative => f.write_char('-'),
            OperatorKind::Plus => f.write_char('+'),
            OperatorKind::Minus => f.write_char('-'),
            OperatorKind::Multiply => f.write_char('*'),
            OperatorKind::Divide => f.write_char('/'),
            OperatorKind::Power => f.write_char('^'),
            OperatorKind::Unwrap => f.write_char('!'),
            OperatorKind::Raise => f.write_char('?'),
            OperatorKind::Celsius => f.write_char('℃'),
            OperatorKind::Fahrenheit => f.write_char('℉'),
            OperatorKind::Transpose => f.write_char('ᵀ'),
            OperatorKind::Transjugate => f.write_char('ᴴ'),
            OperatorKind::Hermitian => f.write_str("Hermitian"),
            OperatorKind::Unbox => f.write_char('*'),
            OperatorKind::Unpack => f.write_str("⁑"),
            OperatorKind::UnpackAll => f.write_char('⁂'),
            OperatorKind::Greater => f.write_char('>'),
            OperatorKind::MuchGreater => f.write_char('≫'),
            OperatorKind::VeryMuchGreater => f.write_char('⋙'),
            OperatorKind::Less => f.write_char('<'),
            OperatorKind::MuchLess => f.write_char('≪'),
            OperatorKind::VeryMuchLess => f.write_char('⋘'),
            OperatorKind::Belongs(v) => match v {
                true => f.write_char('∈'),
                false => f.write_char('∉'),
            },
            OperatorKind::IsA(v) => match v {
                true => f.write_char('⊑'),
                false => f.write_char('⋢'),
            },
            OperatorKind::Equal(v) => match v {
                true => f.write_char('≖'),
                false => f.write_char('≠'),
            },
            OperatorKind::StrictlyEqual(v) => match v {
                true => f.write_char('≡'),
                false => f.write_char('≢'),
            },
            OperatorKind::Inverse => f.write_char('i'),
            OperatorKind::Surd(v) => match v {
                3 => f.write_char('∛'),
                4 => f.write_char('∜'),
                _ => f.write_char('√'),
            },
            OperatorKind::DivideByDecimalPower(v) => match v {
                3 => f.write_char('‰'),
                4 => f.write_char('‱'),
                _ => f.write_char('%'),
            },
        }
    }
}

impl Display for OperatorNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl<E> Display for PrefixNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.operator)
    }
}

impl<E> Display for InfixNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.operator)
    }
}

impl<E> Display for PostfixNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.operator)
    }
}
