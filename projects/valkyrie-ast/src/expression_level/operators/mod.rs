use super::*;

#[cfg(feature = "pretty-print")]
mod display;

mod logic;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ValkyrieOperator {
    /// unassigned operator symbol
    Placeholder,
    /// prefix operator: `!`
    Not,
    /// prefix operator: `+`
    Positive,
    /// prefix operator: `-`
    Negative,
    /// prefix operator: `&`
    Box,
    /// prefix operator: `*`
    Unbox,
    /// prefix operator: `**`
    Unpack,
    /// prefix operator: `**`
    UnpackAll,
    /// prefix operator: `⅟`
    Reciprocal,
    /// prefix operator: `√`, `∛`, `∜`     
    Roots(u8),
    /// infix operator: `=`
    Assign,
    /// binary operator: `+`
    Plus,
    /// binary operator: `+=`
    PlusAssign,
    /// binary operator: `++`
    Concat,
    /// binary operator: `<`
    Less {
        /// binary operator: `<=`, `⩽`
        equal: bool,
    },
    /// binary operator: `>`
    Greater {
        /// binary operator: `>=`, `⩾`
        equal: bool,
    },
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
    /// binary operator: `⊑, ⋢, is, is not`
    Is {
        /// negative operator: `⋢, is not`
        negative: bool,
    },
    /// binary operator: `∈`, `∉`, `in`, `not in`
    In {
        /// negative operator: `∉, not in`
        negative: bool,
    },
    /// binary operator: `∋`, `∌`    
    Contains {
        /// negative operator: `∉, not in`
        negative: bool,
    },
    /// binary operator: `-`
    Minus,
    /// binary operator: `-=`
    MinusAssign,
    /// binary operator: `*`
    Multiply,
    /// binary operator: `/`
    Divide,
    /// binary operator: `%`
    Remider,
    /// binary operator: `/%`
    DivideRemider,
    /// binary operator: `//`
    DivideFloor,
    /// binary operator: `^`
    Power,
    /// binary operator: `√`
    Surd,
    /// binary operator: `&,  |`
    LogicMatrix {
        /// binary operator: `&,  |`
        mask: LogicMatrix,
    },
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
    ///  suffix operator: `\hat`
    Hermitian,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LogicMatrix {
    /// 1. always false
    False = 0b0,
    /// 2. p and q            ∧
    And = 0b1,
    /// 3. p and not q        >
    AndNot = 0b10,
    /// 4. p                  ⊣
    P = 0b11,
    /// 5. not p and q        <
    NotAnd = 0b100,
    /// 6. q                  ⊢
    Q = 0b101,
    /// 7. xor                ≠
    Xor = 0b110,
    /// 8. p or q             ∨
    Or = 0b111,
    /// 9. not p and not q    ⍱
    Nor = 0b1000,
    /// 10. p == q
    Xnor = 0b1001,
    /// 11. not q              ~⊢
    NotQ = 0b1010,
    /// 12. p or not q         ≥
    OrNot = 0b1011,
    /// 13. not p              ~⊣
    NotP = 0b1100,
    /// 14. not p or q         ≤
    NotOr = 0b1101,
    /// 15. not p or not q     ⍲
    Nand = 0b1110,
    /// 16. always true
    True = 0b1111,
}

impl ValkyrieOperator {
    pub fn precedence(&self) -> u32 {
        match self {
            Self::Placeholder => 0,
            //
            Self::Concat => 14000,
            Self::Is { .. } => 14000,
            Self::In { .. } => 14000,
            Self::Contains { .. } => 14000,
            Self::Assign => 14000,
            // prefix - 3
            Self::PlusAssign => 14100,
            Self::MinusAssign => 14100,

            // infix - 3
            Self::LogicMatrix { .. } => 14700,
            Self::Equal(_) => 14700,
            Self::StrictlyEqual(_) => 14700,
            // infix - 2
            Self::Greater { .. } => 14800,
            Self::Less { .. } => 14800,
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
            Self::Remider => 15100,
            Self::DivideRemider => 15100,
            Self::DivideFloor => 15100,
            // infix + 2
            Self::Power => 15200,
            Self::Surd => 15200,
            // prefix + 0
            Self::Not => 25000,
            Self::Positive => 25000,
            Self::Negative => 25000,
            Self::Box => 25000,
            Self::Unbox => 25000,
            Self::Unpack => 25000,
            Self::UnpackAll => 25000,
            Self::Reciprocal => 25000,
            Self::Roots(_) => 25000,
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
            Self::Placeholder => "???",
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
            Self::Remider => "%",
            Self::DivideRemider => "/%",
            Self::DivideFloor => "//",
            Self::Power => "^",
            Self::Surd => "√",
            Self::Optional => "?",
            Self::QuickRaise => "!",
            Self::Celsius => "℃",
            Self::Fahrenheit => "℉",
            Self::Transpose => "ᵀ",
            Self::Transjugate => "ᴴ",
            Self::Hermitian => "Hermitian",
            Self::Box => "&",
            Self::Unbox => "*",
            Self::Unpack => "⁑",
            Self::UnpackAll => "⁂",
            Self::Greater { equal } => match equal {
                true => "⩾",
                false => ">",
            },
            Self::MuchGreater => "≫",
            Self::VeryMuchGreater => "⋙",
            Self::Less { equal } => match equal {
                true => "⩽",
                false => "<",
            },
            Self::MuchLess => "≪",
            Self::VeryMuchLess => "⋘",
            Self::Is { negative } => match negative {
                true => "⋢",
                false => "⊑",
            },
            Self::In { negative } => match negative {
                true => "∉",
                false => "∈",
            },
            Self::Contains { negative } => match negative {
                true => "∌",
                false => "∋",
            },
            Self::Equal(v) => match v {
                true => "≖",
                false => "≠",
            },
            Self::StrictlyEqual(v) => match v {
                true => "≡",
                false => "≢",
            },
            Self::Reciprocal => "⅟",
            Self::Roots(v) => match v {
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
            Self::LogicMatrix { mask } => mask.as_str(),
        }
    }
    /// user input arguments
    pub fn accept_arguments(&self) -> usize {
        match self {
            Self::Plus | Self::Multiply => 0,
            _ => 1,
        }
    }
    /// if this operatr can be override
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
    ///   The operator of the node
    pub operator: OperatorNode,
    ///   The expression that the operator is applied to
    pub base: ExpressionType,
    ///    The span of the operator
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InfixNode {
    ///  The operator of the node
    pub infix: OperatorNode,
    ///  The left hand side of the node
    pub lhs: ExpressionType,
    ///  The right hand side of the node
    pub rhs: ExpressionType,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PostfixNode {
    ///  The operator of the node
    pub operator: OperatorNode,
    ///  The base of the node
    pub base: ExpressionType,
    /// The range of the node
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OperatorNode {
    /// The valid kind of operator
    pub kind: ValkyrieOperator,
    /// The range of the node
    pub span: Range<u32>,
}

impl ValkyrieNode for InfixNode {
    fn get_range(&self) -> Range<u32> {
        let head = self.lhs.get_range().start;
        let tail = self.rhs.get_range().end;
        head..tail
    }
}

impl OperatorNode {
    pub fn new(kind: ValkyrieOperator, span: Range<u32>) -> Self {
        Self { kind, span }
    }
}
