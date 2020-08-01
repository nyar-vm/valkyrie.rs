use super::*;
use crate::PrettyTree;

impl ValkyrieOperator {
    /// Get the normalised string representation of the operator.
    pub fn as_str(&self) -> &'static str {
        match self {
            ValkyrieOperator::Not => "!",
            ValkyrieOperator::Concat => "++",
            ValkyrieOperator::Positive => "+",
            ValkyrieOperator::Negative => "-",
            ValkyrieOperator::Plus => "+",
            ValkyrieOperator::Minus => "-",
            ValkyrieOperator::Multiply => "*",
            ValkyrieOperator::Divide => "/",
            ValkyrieOperator::Power => "^",
            ValkyrieOperator::Unwrap => "!",
            ValkyrieOperator::Raise => "?",
            ValkyrieOperator::Celsius => "℃",
            ValkyrieOperator::Fahrenheit => "℉",
            ValkyrieOperator::Transpose => "ᵀ",
            ValkyrieOperator::Transjugate => "ᴴ",
            ValkyrieOperator::Hermitian => "Hermitian",
            ValkyrieOperator::Unbox => "*",
            ValkyrieOperator::Unpack => "⁑",
            ValkyrieOperator::UnpackAll => "⁂",
            ValkyrieOperator::Greater => ">",
            ValkyrieOperator::MuchGreater => "≫",
            ValkyrieOperator::VeryMuchGreater => "⋙",
            ValkyrieOperator::Less => "<",
            ValkyrieOperator::MuchLess => "≪",
            ValkyrieOperator::VeryMuchLess => "⋘",
            ValkyrieOperator::Belongs(v) => match v {
                true => "∈",
                false => "∉",
            },
            ValkyrieOperator::IsA(v) => match v {
                true => "⊑",
                false => "⋢",
            },
            ValkyrieOperator::Equal(v) => match v {
                true => "≖",
                false => "≠",
            },
            ValkyrieOperator::StrictlyEqual(v) => match v {
                true => "≡",
                false => "≢",
            },
            ValkyrieOperator::Inverse => "i",
            ValkyrieOperator::Surd(v) => match v {
                3 => "∛",
                4 => "∜",
                _ => "√",
            },
            ValkyrieOperator::DivideByDecimalPower(v) => match v {
                3 => "‰",
                4 => "‱",
                _ => "%",
            },
        }
    }
}

impl PrettyPrint for OperatorNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.text(self.kind.as_str()).annotate(allocator.number_style())
    }
}

impl<E: PrettyPrint> PrettyPrint for PrefixNode<E> {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.operator.pretty(allocator).append(self.base.pretty(allocator))
    }
}

impl<E: PrettyPrint> PrettyPrint for InfixNode<E> {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let lhs = self.lhs.pretty(allocator);
        let rhs = self.rhs.pretty(allocator);
        lhs.append(allocator.softline()).append(self.operator.pretty(allocator)).append(allocator.softline()).append(rhs)
    }
}

impl<E: PrettyPrint> PrettyPrint for PostfixNode<E> {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.base.pretty(allocator).append(self.operator.pretty(allocator))
    }
}
