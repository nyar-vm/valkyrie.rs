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
            ValkyrieOperator::PlusAssign => "+=",
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
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.operator(self.kind.as_str())
    }
}

impl<E: PrettyPrint> PrettyPrint for PrefixNode<E> {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.operator.build(allocator).append(self.base.build(allocator))
    }
}

impl<E: PrettyPrint> PrettyPrint for InfixNode<E> {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut items = Vec::with_capacity(5);
        items.push(self.lhs.build(allocator));
        items.push(allocator.space());
        items.push(self.operator.build(allocator));
        items.push(allocator.space());
        items.push(self.rhs.build(allocator));
        allocator.concat(items)
    }
}

impl<E: PrettyPrint> PrettyPrint for PostfixNode<E> {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.base.build(allocator).append(self.operator.build(allocator))
    }
}
