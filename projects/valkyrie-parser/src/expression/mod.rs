use crate::{MainExpressionNode, MainInfixNode, MainPrefixNode, MainSuffixNode, MainTermNode};
use nyar_error::{NyarError, Validation};
use pratt::{Affix, Associativity, PrattParser, Precedence};
use valkyrie_ast::{ExpressionNode, ExpressionType, InfixNode, OperatorNode, PrefixNode, StatementNode, ValkyrieOperator};

impl MainExpressionNode {
    pub fn build(&self) -> Validation<StatementNode> {
        if self.main_infix.is_empty() { todo!() } else { todo!() }
    }
}

struct ExpressionResolver;

enum TokenStream {
    Prefix(ValkyrieOperator),
    Postfix(ValkyrieOperator),
    Infix(ValkyrieOperator),
    Term(MainTermNode),
    Group(Vec<TokenStream>),
}

impl<I> PrattParser<I> for ExpressionResolver
where
    I: Iterator<Item = TokenStream>,
{
    type Error = NyarError;
    type Input = TokenStream;
    type Output = ExpressionType;

    fn query(&mut self, input: &Self::Input) -> Result<Affix, Self::Error> {
        let affix = match input {
            TokenStream::Prefix(v) => Affix::Prefix(Precedence(100u32)),
            TokenStream::Postfix(v) => Affix::Postfix(Precedence(100u32)),
            TokenStream::Infix(v) => Affix::Infix(Precedence(100u32), Associativity::Left),
            TokenStream::Term(_) => Affix::Nilfix,
            TokenStream::Group(_) => Affix::Nilfix,
        };
        Ok(affix)
    }

    fn primary(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn infix(&mut self, lhs: Self::Output, op: Self::Input, rhs: Self::Output) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn prefix(&mut self, op: Self::Input, rhs: Self::Output) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn postfix(&mut self, lhs: Self::Output, op: Self::Input) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}

impl MainPrefixNode {
    pub fn as_operator(&self) -> ValkyrieOperator {
        match self {
            Self::Deconstruct => {
                todo!()
            }
            Self::DeconstructAll => {
                todo!()
            }
            Self::Dereference => {
                todo!()
            }
            Self::Inverse => {
                todo!()
            }
            Self::Negative => {
                todo!()
            }
            Self::Not => {
                todo!()
            }
            Self::Positive => {
                todo!()
            }
            Self::Reference => {
                todo!()
            }
            Self::Root2 => {
                todo!()
            }
            Self::Root3 => {
                todo!()
            }
            Self::Root4 => {
                todo!()
            }
        }
    }
}

impl MainInfixNode {
    pub fn as_operator(&self) -> ValkyrieOperator {
        match self {
            Self::And => {
                todo!()
            }
            Self::Apply2 => {
                todo!()
            }
            Self::Apply3 => {
                todo!()
            }
            Self::Contains => {
                todo!()
            }
            Self::Divide => {
                todo!()
            }
            Self::DivideAssign => {
                todo!()
            }
            Self::EE => {
                todo!()
            }
            Self::EEE => {
                todo!()
            }
            Self::EQ => {
                todo!()
            }
            Self::GE => {
                todo!()
            }
            Self::GEQ => {
                todo!()
            }
            Self::GG => {
                todo!()
            }
            Self::GGE => {
                todo!()
            }
            Self::GGG => {
                todo!()
            }
            Self::In => {
                todo!()
            }
            Self::Is(_) => {
                todo!()
            }
            Self::LE => {
                todo!()
            }
            Self::LEQ => {
                todo!()
            }
            Self::LL => {
                todo!()
            }
            Self::LLE => {
                todo!()
            }
            Self::LLL => {
                todo!()
            }
            Self::Map => {
                todo!()
            }
            Self::Minus => {
                todo!()
            }
            Self::MinusAssign => {
                todo!()
            }
            Self::Multiply => {
                todo!()
            }
            Self::MultiplyAssign => {
                todo!()
            }
            Self::NE => {
                todo!()
            }
            Self::NEE => {
                todo!()
            }
            Self::Nand => {
                todo!()
            }
            Self::Nor => {
                todo!()
            }
            Self::NotContains => {
                todo!()
            }
            Self::NotIn => {
                todo!()
            }
            Self::NotIs => {
                todo!()
            }
            Self::Or => {
                todo!()
            }
            Self::Plus => ValkyrieOperator::Plus,
            Self::PlusAssign => {
                todo!()
            }
            Self::Power => {
                todo!()
            }
            Self::Remainder => {
                todo!()
            }
            Self::RemainderAssign => {
                todo!()
            }
            Self::Surd => {
                todo!()
            }
            Self::Until => {
                todo!()
            }
            Self::UpTo => {
                todo!()
            }
            Self::Xand => {
                todo!()
            }
            Self::Xor => {
                todo!()
            }
        }
    }
}

impl MainSuffixNode {
    pub fn as_operator(&self) -> ValkyrieOperator {
        todo!()
    }
}
