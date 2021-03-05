use crate::{MainExpressionNode, MainFactorNode, MainInfixNode, MainPrefixNode, MainSuffixNode, MainTermNode};
use nyar_error::{NyarError, Success, Validate, Validation};
use pratt::{Affix, Associativity, PrattParser, Precedence};
use valkyrie_ast::{
    BinaryNode, ExpressionNode, ExpressionType, OperatorNode, PostfixNode, StatementNode, UnaryNode, ValkyrieOperator,
};

impl MainExpressionNode {
    pub fn build(&self) -> Validation<ExpressionNode> {
        let mut stream = vec![];
        let (head, rest) = self.main_term.split_first().expect("at least one term");
        head.push_tokens(&mut stream).valid()?;
        for (infix, rhs) in self.main_infix.iter().zip(rest.iter()) {
            stream.push(TokenStream::Infix(infix.as_operator()));
            rhs.push_tokens(&mut stream).valid()?;
        }
        let mut parser = ExpressionResolver;
        let expr = parser.parse(stream.into_iter()).valid()?;
        Success { value: ExpressionNode { type_level: false, body: expr, span: self.span.clone() }, diagnostics: vec![] }
    }
}

impl MainTermNode {
    fn push_tokens(&self, stream: &mut Vec<TokenStream>) -> Validation<()> {
        for i in &self.main_prefix {
            stream.push(TokenStream::Prefix(i.as_operator()))
        }
        let main = self.main_factor.build().valid()?;
        stream.push(TokenStream::Term(main));
        for i in &self.main_suffix {
            stream.push(TokenStream::Postfix(i.as_operator()))
        }
        Success { value: (), diagnostics: vec![] }
    }
}

struct ExpressionResolver;

#[derive(Debug)]
enum TokenStream {
    Prefix(OperatorNode),
    Postfix(OperatorNode),
    Infix(OperatorNode),
    Term(ExpressionType),
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
        };
        Ok(affix)
    }

    fn primary(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        match input {
            TokenStream::Term(v) => Ok(v),
            _ => unreachable!(),
        }
    }

    fn infix(&mut self, lhs: Self::Output, op: Self::Input, rhs: Self::Output) -> Result<Self::Output, Self::Error> {
        match op {
            TokenStream::Infix(v) => Ok(BinaryNode { infix: v, lhs, rhs }.into()),
            _ => unreachable!(),
        }
    }

    fn prefix(&mut self, op: Self::Input, rhs: Self::Output) -> Result<Self::Output, Self::Error> {
        match op {
            TokenStream::Prefix(v) => Ok(UnaryNode { operator: v, base: rhs }.into()),
            _ => unreachable!(),
        }
    }

    fn postfix(&mut self, lhs: Self::Output, op: Self::Input) -> Result<Self::Output, Self::Error> {
        match op {
            TokenStream::Postfix(v) => Ok(PostfixNode { operator: v, base: lhs }.into()),
            _ => unreachable!(),
        }
    }
}

impl MainFactorNode {
    pub fn build(&self) -> Validation<ExpressionType> {
        match self {
            MainFactorNode::Atomic(v) => v.build(),
            MainFactorNode::MainFactor0(_) => {
                todo!()
            }
        }
    }
}
impl MainPrefixNode {
    pub fn as_operator(&self) -> OperatorNode {
        let o = match self.text.as_str() {
            "!" => ValkyrieOperator::Not,
            "+" => ValkyrieOperator::Positive,
            _ => unimplemented!("{} is not a valid prefix operator", self.text),
        };
        OperatorNode { kind: o, span: self.span.clone() }
    }
}

impl MainInfixNode {
    pub fn as_operator(&self) -> OperatorNode {
        let o = match self.text.as_str() {
            s if s.starts_with("is") => ValkyrieOperator::Is { negative: s.ends_with("not") },
            s if s.ends_with("in") => ValkyrieOperator::In { negative: s.ends_with("not") },
            "+" => ValkyrieOperator::Plus,
            _ => unimplemented!("{} is not a valid prefix operator", self.text),
        };
        OperatorNode { kind: o, span: self.span.clone() }
    }
}

impl MainSuffixNode {
    pub fn as_operator(&self) -> OperatorNode {
        todo!()
        // let o = match self.text.as_str() {
        //     "!" => ValkyrieOperator::Not,
        //     _ => unimplemented!("{} is not a valid prefix operator", self.text),
        // };
        // OperatorNode { kind: o, span: self.span.clone() }
    }
}
