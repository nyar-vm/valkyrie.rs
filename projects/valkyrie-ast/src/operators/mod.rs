mod display;
use std::{
    fmt::{Display, Formatter, Write},
    ops::Range,
};

#[derive(Clone, Debug, Eq)]
pub struct ValkyrieOperator {
    pub kind: ValkyrieOperatorKind,
    pub range: Range<usize>,
}

impl PartialEq for ValkyrieOperator {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
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
            ValkyrieOperatorKind::Concat => Precedence(14000),
            ValkyrieOperatorKind::Belongs(_) => Precedence(14000),
            ValkyrieOperatorKind::IsA(_) => Precedence(14000),
            // infix - 3
            ValkyrieOperatorKind::Equal(_) => Precedence(14700),
            ValkyrieOperatorKind::StrictlyEqual(_) => Precedence(14700),
            // infix - 2
            ValkyrieOperatorKind::Greater => Precedence(14800),
            ValkyrieOperatorKind::Less => Precedence(14800),
            // infix - 1
            ValkyrieOperatorKind::MuchLess => Precedence(14900),
            ValkyrieOperatorKind::MuchGreater => Precedence(14900),
            ValkyrieOperatorKind::VeryMuchGreater => Precedence(14950),
            ValkyrieOperatorKind::VeryMuchLess => Precedence(14950),
            // infix + 0
            ValkyrieOperatorKind::Plus => Precedence(15000),
            ValkyrieOperatorKind::Minus => Precedence(15000),
            // infix + 1
            ValkyrieOperatorKind::Multiply => Precedence(15100),
            ValkyrieOperatorKind::Divide => Precedence(15100),
            // infix + 2
            ValkyrieOperatorKind::Power => Precedence(15200),
            // prefix + 0
            ValkyrieOperatorKind::Not => Precedence(25000),
            ValkyrieOperatorKind::Positive => Precedence(25000),
            ValkyrieOperatorKind::Negative => Precedence(25000),
            ValkyrieOperatorKind::Unbox => Precedence(25000),
            ValkyrieOperatorKind::Unpack => Precedence(25000),
            ValkyrieOperatorKind::Inverse => Precedence(25000),
            ValkyrieOperatorKind::Surd(_) => Precedence(25000),
            // postfix + 0
            ValkyrieOperatorKind::Unwrap => Precedence(45000),
            ValkyrieOperatorKind::Raise => Precedence(45000),
            ValkyrieOperatorKind::Celsius => Precedence(45000),
            ValkyrieOperatorKind::Fahrenheit => Precedence(45000),
            ValkyrieOperatorKind::Transpose => Precedence(45000),
            ValkyrieOperatorKind::Transjugate => Precedence(45000),
            ValkyrieOperatorKind::Hermitian => Precedence(45000),
            ValkyrieOperatorKind::DivideByDecimalPower(_) => Precedence(45000),
        }
    }
}
