use super::*;
use std::{char::ParseCharError, str::FromStr};

impl Display for ValkyrieOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Display for ValkyrieOperatorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieOperatorKind::Not => f.write_char('!'),
            ValkyrieOperatorKind::Concat => f.write_str("++"),
            ValkyrieOperatorKind::Positive => f.write_char('+'),
            ValkyrieOperatorKind::Negative => f.write_char('-'),
            ValkyrieOperatorKind::Plus => f.write_char('+'),
            ValkyrieOperatorKind::Minus => f.write_char('-'),
            ValkyrieOperatorKind::Multiply => f.write_char('*'),
            ValkyrieOperatorKind::Divide => f.write_char('/'),
            ValkyrieOperatorKind::Power => f.write_char('^'),
            ValkyrieOperatorKind::Unwrap => f.write_char('!'),
            ValkyrieOperatorKind::Raise => f.write_char('?'),
            ValkyrieOperatorKind::Celsius => f.write_char('℃'),
            ValkyrieOperatorKind::Fahrenheit => f.write_char('℉'),
            ValkyrieOperatorKind::Transpose => f.write_char('ᵀ'),
            ValkyrieOperatorKind::Transjugate => f.write_char('ᴴ'),
            ValkyrieOperatorKind::Hermitian => f.write_str("Hermitian"),
            ValkyrieOperatorKind::Unbox => f.write_char('*'),
            ValkyrieOperatorKind::Unpack => f.write_str("⁑"),
            ValkyrieOperatorKind::UnpackAll => f.write_char('⁂'),
            ValkyrieOperatorKind::Greater => f.write_char('>'),
            ValkyrieOperatorKind::MuchGreater => f.write_char('≫'),
            ValkyrieOperatorKind::VeryMuchGreater => f.write_char('⋙'),
            ValkyrieOperatorKind::Less => f.write_char('<'),
            ValkyrieOperatorKind::MuchLess => f.write_char('≪'),
            ValkyrieOperatorKind::VeryMuchLess => f.write_char('⋘'),
            ValkyrieOperatorKind::Belongs(v) => match v {
                true => f.write_char('∈'),
                false => f.write_char('∉'),
            },
            ValkyrieOperatorKind::IsA(v) => match v {
                true => f.write_char('⊑'),
                false => f.write_char('⋢'),
            },
            ValkyrieOperatorKind::Equal(v) => match v {
                true => f.write_char('≖'),
                false => f.write_char('≠'),
            },
            ValkyrieOperatorKind::StrictlyEqual(v) => match v {
                true => f.write_char('≡'),
                false => f.write_char('≢'),
            },
            ValkyrieOperatorKind::Inverse => f.write_char('i'),
            ValkyrieOperatorKind::Surd(v) => match v {
                3 => f.write_char('∛'),
                4 => f.write_char('∜'),
                _ => f.write_char('√'),
            },
            ValkyrieOperatorKind::DivideByDecimalPower(v) => match v {
                3 => f.write_char('‰'),
                4 => f.write_char('‱'),
                _ => f.write_char('%'),
            },
        }
    }
}
