use super::*;
use std::fmt::Write;

impl Display for ValkyrieOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieOperator::Not => f.write_char('!'),
            ValkyrieOperator::Positive => f.write_char('+'),
            ValkyrieOperator::Negative => f.write_char('-'),
            ValkyrieOperator::Plus => f.write_char('+'),
            ValkyrieOperator::Minus => f.write_char('-'),
            ValkyrieOperator::Mul => f.write_char('*'),
            ValkyrieOperator::Div => f.write_char('/'),
            ValkyrieOperator::Pow => f.write_char('^'),
            ValkyrieOperator::Eq => f.write_char('='),
            ValkyrieOperator::Raise => f.write_char('?'),
            ValkyrieOperator::Celsius => f.write_char('℃'),
            ValkyrieOperator::Fahrenheit => f.write_char('℉'),
            ValkyrieOperator::Transpose => f.write_char('ᵀ'),
            ValkyrieOperator::Transjugate => f.write_char('ᴴ'),
            ValkyrieOperator::Hermitian => f.write_str("Hermitian"),
        }
    }
}
