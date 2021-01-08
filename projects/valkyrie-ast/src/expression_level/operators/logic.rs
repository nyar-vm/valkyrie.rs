use super::*;

impl LogicMatrix {
    /// Use as a binary operator
    pub fn perform_logic(&self, p: bool, q: bool) -> bool {
        match self {
            Self::False => false,
            Self::And => p && q,
            Self::AndNot => p && !q,
            Self::P => p,
            Self::NotAnd => !p && q,
            Self::Q => q,
            Self::Xor => p ^ q,
            Self::Or => p || q,
            Self::Nor => !p && !q,
            Self::Xnor => p == q,
            Self::NotQ => !q,
            Self::OrNot => p || !q,
            Self::NotP => !p,
            Self::NotOr => !p || q,
            Self::Nand => !p || !q,
            Self::True => true,
        }
    }
    /// Use on byte, bit by bit
    pub fn perform_byte(&self, p: u8, q: u8) -> u8 {
        match self {
            Self::False => 0,
            Self::And => p & q,
            Self::AndNot => p & !q,
            Self::P => p,
            Self::NotAnd => !p & q,
            Self::Q => q,
            Self::Xor => p ^ q,
            Self::Or => p | q,
            Self::Nor => !(p | q),
            Self::Xnor => p ^ q,
            Self::NotQ => !q,
            Self::OrNot => p | !q,
            Self::NotP => !p,
            Self::NotOr => !(p | q),
            Self::Nand => !(p & q),
            Self::True => 255,
        }
    }

    /// Convert to string if is a binary operator
    pub fn as_str(&self) -> &'static str {
        match self {
            LogicMatrix::False => "_",
            LogicMatrix::And => "∧",
            LogicMatrix::AndNot => "",
            LogicMatrix::P => "⊣",
            LogicMatrix::NotAnd => "",
            LogicMatrix::Q => "⊢",
            LogicMatrix::Xor => "≠",
            LogicMatrix::Or => "∨",
            LogicMatrix::Nor => "⍱",
            LogicMatrix::Xnor => "==",
            LogicMatrix::NotQ => "",
            LogicMatrix::OrNot => "",
            LogicMatrix::NotP => "",
            LogicMatrix::NotOr => "",
            LogicMatrix::Nand => "⍲",
            LogicMatrix::True => "_",
        }
    }
}
