use super::*;
mod display;

#[derive(Clone, Debug, Eq, Hash)]
pub struct ValkyrieOperator {
    pub kind: ValkyrieOperatorKind,
    pub range: Range<usize>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ValkyrieOperatorKind {
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

impl ValkyrieOperator {
    pub fn new(kind: ValkyrieOperatorKind, range: Range<usize>) -> Self {
        Self { kind, range }
    }
}

impl ValkyrieOperatorKind {
    pub fn precedence(&self) -> u32 {
        match self {
            //
            ValkyrieOperatorKind::Concat => 14000,
            ValkyrieOperatorKind::Belongs(_) => 14000,
            ValkyrieOperatorKind::IsA(_) => 14000,
            // infix - 3
            ValkyrieOperatorKind::Equal(_) => 14700,
            ValkyrieOperatorKind::StrictlyEqual(_) => 14700,
            // infix - 2
            ValkyrieOperatorKind::Greater => 14800,
            ValkyrieOperatorKind::Less => 14800,
            // infix - 1
            ValkyrieOperatorKind::MuchLess => 14900,
            ValkyrieOperatorKind::MuchGreater => 14900,
            ValkyrieOperatorKind::VeryMuchGreater => 14950,
            ValkyrieOperatorKind::VeryMuchLess => 14950,
            // infix + 0
            ValkyrieOperatorKind::Plus => 15000,
            ValkyrieOperatorKind::Minus => 15000,
            // infix + 1
            ValkyrieOperatorKind::Multiply => 15100,
            ValkyrieOperatorKind::Divide => 15100,
            // infix + 2
            ValkyrieOperatorKind::Power => 15200,
            // prefix + 0
            ValkyrieOperatorKind::Not => 25000,
            ValkyrieOperatorKind::Positive => 25000,
            ValkyrieOperatorKind::Negative => 25000,
            ValkyrieOperatorKind::Unbox => 25000,
            ValkyrieOperatorKind::Unpack => 25000,
            ValkyrieOperatorKind::Inverse => 25000,
            ValkyrieOperatorKind::Surd(_) => 25000,
            // postfix + 0
            ValkyrieOperatorKind::Unwrap => 45000,
            ValkyrieOperatorKind::Raise => 45000,
            ValkyrieOperatorKind::Celsius => 45000,
            ValkyrieOperatorKind::Fahrenheit => 45000,
            ValkyrieOperatorKind::Transpose => 45000,
            ValkyrieOperatorKind::Transjugate => 45000,
            ValkyrieOperatorKind::Hermitian => 45000,
            ValkyrieOperatorKind::DivideByDecimalPower(_) => 45000,
        }
    }
}
