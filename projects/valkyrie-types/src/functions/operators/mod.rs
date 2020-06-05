use std::fmt::{Display, Formatter, Write};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ValkyrieOperator {
    /// prefix operator: `!`
    Not,
    /// prefix operator: `+`
    Positive,
    /// prefix operator: `-`
    Negative,
    /// prefix operator: `*`
    Unbox,
    /// prefix operator: `**`
    Unpack,
    /// prefix operator: `**`
    UnpackAll,
    /// prefix operator: `⅟`
    Inverse,
    /// prefix operator: `⅟`
    Surd(u8),
    /// binary operator: `+`
    Plus,
    /// binary operator: `++`
    Concat,
    /// binary operator: `>`
    Greater,
    /// binary operator: `<`
    Less,
    /// binary operator: `≫`, `>>`
    MuchGreater,
    /// binary operator:
    MuchLess,
    /// binary operator: `⋙`, `>>>`
    VeryMuchGreater,
    /// binary operator:
    VeryMuchLess,
    /// binary operator: `≡`, `≢`
    Equal(bool),
    /// binary operator:
    StrictlyEqual(bool),
    /// binary operator: `in, not in`
    Belongs(bool),
    /// binary operator: `⊑, ⋢, is, is not`
    IsA(bool),

    /// binary operator: `-`
    Minus,
    /// binary operator: `*`
    Multiply,
    /// binary operator: `/`
    Divide,
    /// binary operator: `^`
    Power,
    /// suffix operator: `!`
    Unwrap,
    /// suffix operator: `?`
    Raise,
    /// suffix operator: `℃`
    Celsius,
    /// suffix operator: `℉`
    Fahrenheit,
    /// suffix operator: `a%, b‰, c‱`
    DivideByDecimalPower(u8),
    /// suffix operator: `ᵀ`, `\^T`, `\transpose`
    Transpose,
    /// suffix operator: `ᴴ`, `\^H`, `\conjugate_transpose
    Transjugate,
    Hermitian,
}

impl Display for ValkyrieOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl ValkyrieOperator {
    pub fn precedence(&self) -> u32 {
        match self {
            //
            ValkyrieOperator::Concat => 14000,
            ValkyrieOperator::Belongs(_) => 14000,
            ValkyrieOperator::IsA(_) => 14000,
            // infix - 3
            ValkyrieOperator::Equal(_) => 14700,
            ValkyrieOperator::StrictlyEqual(_) => 14700,
            // infix - 2
            ValkyrieOperator::Greater => 14800,
            ValkyrieOperator::Less => 14800,
            // infix - 1
            ValkyrieOperator::MuchLess => 14900,
            ValkyrieOperator::MuchGreater => 14900,
            ValkyrieOperator::VeryMuchGreater => 14950,
            ValkyrieOperator::VeryMuchLess => 14950,
            // infix + 0
            ValkyrieOperator::Plus => 15000,
            ValkyrieOperator::Minus => 15000,
            // infix + 1
            ValkyrieOperator::Multiply => 15100,
            ValkyrieOperator::Divide => 15100,
            // infix + 2
            ValkyrieOperator::Power => 15200,
            // prefix + 0
            ValkyrieOperator::Not => 25000,
            ValkyrieOperator::Positive => 25000,
            ValkyrieOperator::Negative => 25000,
            ValkyrieOperator::Unbox => 25000,
            ValkyrieOperator::Unpack => 25000,
            ValkyrieOperator::UnpackAll => 25000,
            ValkyrieOperator::Inverse => 25000,
            ValkyrieOperator::Surd(_) => 25000,
            // postfix + 0
            ValkyrieOperator::Unwrap => 45000,
            ValkyrieOperator::Raise => 45000,
            ValkyrieOperator::Celsius => 45000,
            ValkyrieOperator::Fahrenheit => 45000,
            ValkyrieOperator::Transpose => 45000,
            ValkyrieOperator::Transjugate => 45000,
            ValkyrieOperator::Hermitian => 45000,
            ValkyrieOperator::DivideByDecimalPower(_) => 45000,
        }
    }
    pub fn accept_arguments(&self) -> usize {
        match self {
            ValkyrieOperator::Plus => 0,
            ValkyrieOperator::Multiply => 0,
            _ => 1,
        }
    }
}
