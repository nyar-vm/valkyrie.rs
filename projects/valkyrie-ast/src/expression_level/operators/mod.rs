use super::*;
mod display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrefixNode<E> {
    pub operator: OperatorNode,
    pub body: E,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InfixNode<E> {
    pub operator: OperatorNode,
    pub lhs: E,
    pub rhs: E,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PostfixNode<E> {
    pub operator: OperatorNode,
    pub body: E,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, Eq, Hash)]
pub struct OperatorNode {
    pub kind: OperatorKind,
    pub range: Range<usize>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum OperatorKind {
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

impl OperatorNode {
    pub fn new(kind: OperatorKind, range: Range<usize>) -> Self {
        Self { kind, range }
    }
}

impl OperatorKind {
    pub fn precedence(&self) -> u32 {
        match self {
            //
            OperatorKind::Concat => 14000,
            OperatorKind::Belongs(_) => 14000,
            OperatorKind::IsA(_) => 14000,
            // infix - 3
            OperatorKind::Equal(_) => 14700,
            OperatorKind::StrictlyEqual(_) => 14700,
            // infix - 2
            OperatorKind::Greater => 14800,
            OperatorKind::Less => 14800,
            // infix - 1
            OperatorKind::MuchLess => 14900,
            OperatorKind::MuchGreater => 14900,
            OperatorKind::VeryMuchGreater => 14950,
            OperatorKind::VeryMuchLess => 14950,
            // infix + 0
            OperatorKind::Plus => 15000,
            OperatorKind::Minus => 15000,
            // infix + 1
            OperatorKind::Multiply => 15100,
            OperatorKind::Divide => 15100,
            // infix + 2
            OperatorKind::Power => 15200,
            // prefix + 0
            OperatorKind::Not => 25000,
            OperatorKind::Positive => 25000,
            OperatorKind::Negative => 25000,
            OperatorKind::Unbox => 25000,
            OperatorKind::Unpack => 25000,
            OperatorKind::UnpackAll => 25000,
            OperatorKind::Inverse => 25000,
            OperatorKind::Surd(_) => 25000,
            // postfix + 0
            OperatorKind::Unwrap => 45000,
            OperatorKind::Raise => 45000,
            OperatorKind::Celsius => 45000,
            OperatorKind::Fahrenheit => 45000,
            OperatorKind::Transpose => 45000,
            OperatorKind::Transjugate => 45000,
            OperatorKind::Hermitian => 45000,
            OperatorKind::DivideByDecimalPower(_) => 45000,
        }
    }
}
