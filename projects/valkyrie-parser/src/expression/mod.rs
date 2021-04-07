use crate::{
    helpers::ProgramContext, DotCallItemNode, ExpressionStatementNode, InlineSuffixNode, MainExpressionNode, MainFactorNode,
    MainInfixNode, MainPrefixNode, MainSuffixNode, MainTermNode, SuffixOperatorNode, TypeExpressionNode, TypeFactorNode,
    TypeInfixNode, TypePrefixNode, TypeSuffixNode, TypeTermNode,
};
use nyar_error::{NyarError, Success, Validate, Validation};
use pratt::{Affix, PrattParser, Precedence};
use std::str::FromStr;
use valkyrie_ast::{
    ApplyCallNode, ArgumentsList, BinaryNode, DotCallNode, DotCallTerm, ExpressionNode, ExpressionType, GenericCallNode,
    OperatorNode, SubscriptCallNode, TupleTermNode, UnaryNode, ValkyrieOperator,
};

mod dot_call;
mod generic_call;

impl ExpressionStatementNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ExpressionNode> {
        let expr = self.main_expression.build(ctx)?;
        let eos = self.eos.is_some();
        Success { value: ExpressionNode { omit: eos, body: expr, span: self.span.clone() }, diagnostics: vec![] }
    }
}

impl MainExpressionNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ExpressionType> {
        let mut stream = vec![];
        let (head, rest) = self.main_term.split_first().expect("at least one term");
        head.push_tokens(&mut stream, ctx)?;
        for (infix, rhs) in self.main_infix.iter().zip(rest.iter()) {
            stream.push(TokenStream::Infix(infix.as_operator()));
            rhs.push_tokens(&mut stream, ctx).valid()?;
        }
        let mut parser = ExpressionResolver;
        let expr = parser.parse(stream.into_iter()).valid()?;
        Success { value: expr, diagnostics: vec![] }
    }
}
impl TypeExpressionNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ExpressionType> {
        let mut stream = vec![];
        let (head, rest) = self.type_term.split_first().expect("at least one term");
        head.push_tokens(&mut stream, ctx)?;
        for (infix, rhs) in self.type_infix.iter().zip(rest.iter()) {
            stream.push(TokenStream::Infix(infix.as_operator()));
            rhs.push_tokens(&mut stream, ctx).valid()?;
        }
        let mut parser = ExpressionResolver;
        let expr = parser.parse(stream.into_iter()).valid()?;
        Success { value: expr, diagnostics: vec![] }
    }
}

impl MainTermNode {
    fn push_tokens(&self, stream: &mut Vec<TokenStream>, ctx: &ProgramContext) -> Validation<()> {
        for i in &self.main_prefix {
            stream.push(TokenStream::Prefix(i.as_operator()))
        }
        let main = self.main_factor.build(ctx).valid()?;
        stream.push(TokenStream::Term(main));
        for i in &self.main_suffix {
            stream.push(i.as_token(ctx)?)
        }
        Success { value: (), diagnostics: vec![] }
    }
}
impl TypeTermNode {
    fn push_tokens(&self, stream: &mut Vec<TokenStream>, ctx: &ProgramContext) -> Validation<()> {
        for i in &self.type_prefix {
            stream.push(TokenStream::Prefix(i.as_operator()))
        }
        let main = self.main_factor.build(ctx).valid()?;
        stream.push(TokenStream::Term(main));
        for i in &self.type_suffix {
            stream.push(i.as_token(ctx)?)
        }
        Success { value: (), diagnostics: vec![] }
    }
}

struct ExpressionResolver;

#[derive(Debug)]
enum TokenStream {
    Prefix(OperatorNode),
    Infix(OperatorNode),
    Term(ExpressionType),
    Postfix(OperatorNode),
    Subscript(SubscriptCallNode),
    Generic(GenericCallNode),
    Apply(ApplyCallNode),
    Dot(DotCallNode),
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
            TokenStream::Prefix(v) => Affix::Prefix(v.kind.precedence()),
            TokenStream::Infix(v) => Affix::Infix(v.kind.precedence(), v.kind.associativity()),
            TokenStream::Term(_) => Affix::Nilfix,
            TokenStream::Postfix(v) => Affix::Postfix(v.kind.precedence()),
            _ => Affix::Postfix(Precedence(10000)),
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
            TokenStream::Postfix(v) => Ok(UnaryNode { operator: v, base: lhs }.into()),
            TokenStream::Subscript(call) => Ok(call.with_base(lhs).into()),
            TokenStream::Apply(call) => Ok(call.with_base(lhs).into()),
            TokenStream::Dot(call) => Ok(call.with_base(lhs).into()),
            TokenStream::Generic(call) => Ok(call.with_base(lhs).into()),
            _ => unreachable!(),
        }
    }
}

impl MainFactorNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ExpressionType> {
        match self {
            Self::Leading(v) => v.build(ctx),
            Self::GroupFactor(v) => v.main_expression.build(ctx),
            Self::NewStatement(v) => v.build(ctx).map(Into::into),
            Self::ObjectStatement(v) => v.build(ctx).map(Into::into),
            Self::TryStatement(v) => v.build(ctx).map(Into::into),
            Self::MatchExpression(v) => v.build(ctx).map(Into::into),
        }
    }
}
impl TypeFactorNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ExpressionType> {
        match self {
            Self::Leading(v) => v.build(ctx),
            Self::TypeFactor0(_) => {
                todo!()
            }
        }
    }
}

impl MainPrefixNode {
    pub fn as_operator(&self) -> OperatorNode {
        use ValkyrieOperator::*;
        let o = match self.text.as_str() {
            "!" => Not,
            "+" => Positive,
            "-" => Negative,
            "*" => Unbox,
            "⅟" => Reciprocal,
            "√" => Roots(2),
            "∛" => Roots(3),
            "∜" => Roots(4),
            ".." => Unpack { level: 2 },
            "..." => Unpack { level: 3 },
            _ => unimplemented!("{} is a unknown prefix operator", self.text),
        };
        OperatorNode { kind: o, span: self.span.clone() }
    }
}
impl TypePrefixNode {
    pub fn as_operator(&self) -> OperatorNode {
        use ValkyrieOperator::*;
        let o = match self.text.as_str() {
            "!" => Not,
            _ => unimplemented!("{} is a unknown prefix operator", self.text),
        };
        OperatorNode { kind: o, span: self.span.clone() }
    }
}
impl MainInfixNode {
    pub fn as_operator(&self) -> OperatorNode {
        use valkyrie_ast::LogicMatrix;
        use ValkyrieOperator::*;
        let o = match self.text.as_str() {
            s if s.starts_with("is") => Is { negative: s.ends_with("not") },
            s if s.ends_with("in") => In { negative: s.ends_with("not") },
            "∈" | "∊" => In { negative: false },
            "∉" => In { negative: true },
            "∋" => Contains { negative: false },
            "∌" => Contains { negative: true },

            "+" => Plus,
            "-" => Minus,
            "*" => Multiply,
            "/" => Divide,
            "%" => Remider,
            "^" => Power,
            "=" => Assign { monadic: false },
            "?=" => Assign { monadic: true },
            "==" => Equal { negative: false },
            "≠" | "!=" => Equal { negative: true },
            "≡" | "===" => StrictlyEqual { negative: false },
            "≢" | "!==" | "=!=" => StrictlyEqual { negative: true },
            ">" => Greater { equal: false },
            "≥" | ">=" => Greater { equal: true },
            "≫" | ">>" => MuchGreater,
            "⋙" | ">>>" => VeryMuchGreater,
            ">>=" => Placeholder,
            "<" => Less { equal: false },
            "≤" | "<=" => Less { equal: true },
            "≪" | "<<" => MuchLess,
            "⋘" | "<<<" => VeryMuchLess,
            "<<=" => Placeholder,
            // logic operators
            "∧" | "&&" => LogicMatrix::And.into(),
            "⊼" => LogicMatrix::Nand.into(),
            "⩟" => LogicMatrix::Xnor.into(), // aka. xand
            "∨" | "||" => LogicMatrix::Or.into(),
            "⊽" => LogicMatrix::Nor.into(),
            "⊻" => LogicMatrix::Xor.into(),
            _ => unimplemented!("{} is a unknown infix operator", self.text),
        };
        OperatorNode { kind: o, span: self.span.clone() }
    }
}
impl TypeInfixNode {
    pub fn as_operator(&self) -> OperatorNode {
        use valkyrie_ast::LogicMatrix;
        use ValkyrieOperator::*;
        let o = match self.text.as_str() {
            s if s.starts_with("is") => Is { negative: s.ends_with("not") },
            _ => unimplemented!("{} is a unknown infix operator", self.text),
        };
        OperatorNode { kind: o, span: self.span.clone() }
    }
}
impl SuffixOperatorNode {
    pub fn as_operator(&self) -> OperatorNode {
        use ValkyrieOperator::*;
        let o = match self.text.as_str() {
            "!" => QuickRaise,
            "℃" => Celsius,
            "℉" => Fahrenheit,
            "%" => DivideByDecimalPower(2),
            "‰" => DivideByDecimalPower(3),
            "‱" => DivideByDecimalPower(4),
            _ => unimplemented!("{} is a unknown suffix operator", self.text),
        };
        OperatorNode { kind: o, span: self.span.clone() }
    }
}

impl MainSuffixNode {
    fn as_token(&self, ctx: &ProgramContext) -> Validation<TokenStream> {
        let token = match self {
            MainSuffixNode::InlineSuffix(v) => v.as_token(ctx)?,
        };
        Success { value: token, diagnostics: vec![] }
    }
}

impl InlineSuffixNode {
    fn as_token(&self, ctx: &ProgramContext) -> Validation<TokenStream> {
        let token = match self {
            InlineSuffixNode::InlineSuffix0(v) => TokenStream::Postfix(v.as_operator()),
            InlineSuffixNode::RangeCall(v) => TokenStream::Subscript(v.build(ctx)?),
            InlineSuffixNode::TupleCall(v) => TokenStream::Apply(v.build(ctx)?),
            InlineSuffixNode::DotCall(v) => TokenStream::Dot(v.build(ctx)?),
            InlineSuffixNode::GenericCall(v) => TokenStream::Generic(v.build(ctx)?),
        };
        Success { value: token, diagnostics: vec![] }
    }
}

impl TypeSuffixNode {
    fn as_token(&self, ctx: &ProgramContext) -> Validation<TokenStream> {
        let token = match self {
            TypeSuffixNode::GenericHide(v) => TokenStream::Generic(v.build(ctx)?),
            TypeSuffixNode::Option => {
                todo!()
            }
        };
        Success { value: token, diagnostics: vec![] }
    }
}
