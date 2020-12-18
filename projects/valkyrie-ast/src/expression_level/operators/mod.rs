use super::*;
#[cfg(feature = "pretty-print")]
mod display;

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
    /// infix operator: `=`
    Assign,
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
    /// binary operator: `-=`
    MinusAssign,
    /// binary operator: `*`
    Multiply,
    /// binary operator: `/`
    Divide,
    /// binary operator: `^`
    Power,
    /// suffix operator: `|`
    BitOr,
    /// suffix operator: `&`
    BitAnd,
    /// suffix operator: `?`
    Optional,
    /// suffix operator: `!`
    QuickRaise,
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
            Self::Concat => 14000,
            Self::Belongs(_) => 14000,
            Self::IsA(_) => 14000,

            Self::Assign => 14000,
            // prefix - 3
            Self::PlusAssign => 14100,
            Self::MinusAssign => 14100,

            // infix - 3
            Self::BitOr => 14700,
            Self::BitAnd => 14700,
            Self::Equal(_) => 14700,
            Self::StrictlyEqual(_) => 14700,
            // infix - 2
            Self::Greater => 14800,
            Self::Less => 14800,
            // infix - 1
            Self::MuchLess => 14900,
            Self::MuchGreater => 14900,
            Self::VeryMuchGreater => 14950,
            Self::VeryMuchLess => 14950,
            // infix + 0
            Self::Plus => 15000,
            Self::Minus => 15000,
            // infix + 1
            Self::Multiply => 15100,
            Self::Divide => 15100,
            // infix + 2
            Self::Power => 15200,
            // prefix + 0
            Self::Not => 25000,
            Self::Positive => 25000,
            Self::Negative => 25000,
            Self::Unbox => 25000,
            Self::Unpack => 25000,
            Self::UnpackAll => 25000,
            Self::Inverse => 25000,
            Self::Surd(_) => 25000,
            // postfix + 0
            Self::Optional => 45000,
            Self::QuickRaise => 45000,
            Self::Celsius => 45000,
            Self::Fahrenheit => 45000,
            Self::Transpose => 45000,
            Self::Transjugate => 45000,
            Self::Hermitian => 45000,
            Self::DivideByDecimalPower(_) => 45000,
        }
    }
    /// Get the normalised string representation of the operator.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Not => "!",
            Self::Concat => "++",
            Self::Positive => "+",
            Self::Negative => "-",
            Self::Plus => "+",
            Self::PlusAssign => "+=",
            Self::Minus => "-",
            Self::MinusAssign => "-=",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Power => "^",
            Self::Optional => "?",
            Self::QuickRaise => "!",
            Self::Celsius => "℃",
            Self::Fahrenheit => "℉",
            Self::Transpose => "ᵀ",
            Self::Transjugate => "ᴴ",
            Self::Hermitian => "Hermitian",
            Self::Unbox => "*",
            Self::Unpack => "⁑",
            Self::UnpackAll => "⁂",
            Self::Greater => ">",
            Self::MuchGreater => "≫",
            Self::VeryMuchGreater => "⋙",
            Self::Less => "<",
            Self::MuchLess => "≪",
            Self::VeryMuchLess => "⋘",
            Self::Belongs(v) => match v {
                true => "∈",
                false => "∉",
            },
            Self::IsA(v) => match v {
                true => "⊑",
                false => "⋢",
            },
            Self::Equal(v) => match v {
                true => "≖",
                false => "≠",
            },
            Self::StrictlyEqual(v) => match v {
                true => "≡",
                false => "≢",
            },
            Self::Inverse => "i",
            Self::Surd(v) => match v {
                3 => "∛",
                4 => "∜",
                _ => "√",
            },
            Self::DivideByDecimalPower(v) => match v {
                3 => "‰",
                4 => "‱",
                _ => "%",
            },
            Self::Assign => "=",
            Self::BitOr => "|",
            Self::BitAnd => "&",
        }
    }
    pub fn accept_arguments(&self) -> usize {
        match self {
            Self::Plus | Self::Multiply => 0,
            _ => 1,
        }
    }
    pub fn overrideable(&self) -> bool {
        match self {
            Self::Equal(false) => false,
            Self::StrictlyEqual(false) => false,
            _ => true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrefixNode {
    pub operator: OperatorNode,
    pub base: ExpressionType,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InfixNode {
    pub operator: OperatorNode,
    pub lhs: ExpressionType,
    pub rhs: ExpressionType,
    /// The range of the node
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PostfixNode {
    pub operator: OperatorNode,
    pub base: ExpressionType,
    /// The range of the node
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OperatorNode {
    pub kind: ValkyrieOperator,
    /// The range of the node
    pub span: Range<u32>,
}

impl OperatorNode {
    pub fn new(kind: ValkyrieOperator, span: Range<u32>) -> Self {
        Self { kind, span }
    }
}
