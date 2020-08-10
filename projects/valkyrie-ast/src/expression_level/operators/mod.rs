mod display;
use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// binary operator: `+=`
    PlusAssign,
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
    pub fn precedence(&self) -> u32 {
        match self {
            //
            ValkyrieOperator::Concat => 14000,
            ValkyrieOperator::Belongs(_) => 14000,
            ValkyrieOperator::IsA(_) => 14000,

            // prefix - 3
            ValkyrieOperator::PlusAssign => 14100,

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
            ValkyrieOperator::Plus | ValkyrieOperator::Multiply => 0,
            _ => 1,
        }
    }
    pub fn overrideable(&self) -> bool {
        match self {
            ValkyrieOperator::Equal(false) => false,
            ValkyrieOperator::StrictlyEqual(false) => false,
            _ => true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrefixNode {
    pub operator: OperatorNode,
    pub base: ExpressionBody,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InfixNode {
    pub operator: OperatorNode,
    pub lhs: ExpressionBody,
    pub rhs: ExpressionBody,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PostfixNode {
    pub operator: OperatorNode,
    pub base: ExpressionBody,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OperatorNode {
    pub kind: ValkyrieOperator,
    pub span: Range<u32>,
}

impl OperatorNode {
    pub fn new(kind: ValkyrieOperator, span: Range<u32>) -> Self {
        Self { kind, span }
    }
}
