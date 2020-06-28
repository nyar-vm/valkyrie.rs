use super::*;

impl Display for ValkyrieOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ValkyrieOperator::Not => f.write_char('!'),
            ValkyrieOperator::Concat => f.write_str("++"),
            ValkyrieOperator::Positive => f.write_char('+'),
            ValkyrieOperator::Negative => f.write_char('-'),
            ValkyrieOperator::Plus => f.write_char('+'),
            ValkyrieOperator::Minus => f.write_char('-'),
            ValkyrieOperator::Multiply => f.write_char('*'),
            ValkyrieOperator::Divide => f.write_char('/'),
            ValkyrieOperator::Power => f.write_char('^'),
            ValkyrieOperator::Unwrap => f.write_char('!'),
            ValkyrieOperator::Raise => f.write_char('?'),
            ValkyrieOperator::Celsius => f.write_char('℃'),
            ValkyrieOperator::Fahrenheit => f.write_char('℉'),
            ValkyrieOperator::Transpose => f.write_char('ᵀ'),
            ValkyrieOperator::Transjugate => f.write_char('ᴴ'),
            ValkyrieOperator::Hermitian => f.write_str("Hermitian"),
            ValkyrieOperator::Unbox => f.write_char('*'),
            ValkyrieOperator::Unpack => f.write_str("⁑"),
            ValkyrieOperator::UnpackAll => f.write_char('⁂'),
            ValkyrieOperator::Greater => f.write_char('>'),
            ValkyrieOperator::MuchGreater => f.write_char('≫'),
            ValkyrieOperator::VeryMuchGreater => f.write_char('⋙'),
            ValkyrieOperator::Less => f.write_char('<'),
            ValkyrieOperator::MuchLess => f.write_char('≪'),
            ValkyrieOperator::VeryMuchLess => f.write_char('⋘'),
            ValkyrieOperator::Belongs(v) => match v {
                true => f.write_char('∈'),
                false => f.write_char('∉'),
            },
            ValkyrieOperator::IsA(v) => match v {
                true => f.write_char('⊑'),
                false => f.write_char('⋢'),
            },
            ValkyrieOperator::Equal(v) => match v {
                true => f.write_char('≖'),
                false => f.write_char('≠'),
            },
            ValkyrieOperator::StrictlyEqual(v) => match v {
                true => f.write_char('≡'),
                false => f.write_char('≢'),
            },
            ValkyrieOperator::Inverse => f.write_char('i'),
            ValkyrieOperator::Surd(v) => match v {
                3 => f.write_char('∛'),
                4 => f.write_char('∜'),
                _ => f.write_char('√'),
            },
            ValkyrieOperator::DivideByDecimalPower(v) => match v {
                3 => f.write_char('‰'),
                4 => f.write_char('‱'),
                _ => f.write_char('%'),
            },
        }
    }
}

impl Display for OperatorNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl<E: Display> Display for PrefixNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}", self.operator, self.body)
    }
}

impl<E: Display> Display for InfixNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.operator, self.rhs)
    }
}

impl<E: Display> Display for PostfixNode<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}", self.body, self.operator)
    }
}
