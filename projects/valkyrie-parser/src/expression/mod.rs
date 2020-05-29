use crate::{helpers::ignore, operators::ValkyrieInfix};
use lispify::{Lisp, Lispify};
use pex::{ParseResult, ParseState, StopBecause};
use pratt::{Affix, PrattError, PrattParser};
use std::{fmt::Debug, ops::Range};
mod display;
mod parser;
use crate::{
    helpers::parse_value,
    operators::{ValkyriePrefix, ValkyrieSuffix},
    traits::ThisParser,
};
use pex::helpers::make_from_str;
use std::str::FromStr;
use valkyrie_ast::{NamepathNode, NumberLiteralNode, StringLiteralNode, ValkyrieOperator, ValkyrieOperatorKind};

/// A resolver
#[derive(Default)]
pub struct ExpressionResolver {}

impl ExpressionResolver {
    pub fn resolve(&self, stream: Vec<ExpressionStream>) -> Result<ValkyrieExpression, StopBecause> {
        // println!("stream: {stream:#?}");
        let mut effect = ExpressionResolver {};
        match effect.parse(stream.into_iter()) {
            Ok(o) => Ok(o),
            Err(e) => say_stop_reason(e),
        }
    }
}

fn say_stop_reason<T>(e: PrattError<ExpressionStream, StopBecause>) -> Result<T, StopBecause> {
    match e {
        PrattError::UserError(e) => Err(e),
        PrattError::EmptyInput => unreachable!(),
        PrattError::UnexpectedNilfix(_) => unreachable!(),
        PrattError::UnexpectedPrefix(_) => unreachable!(),
        PrattError::UnexpectedInfix(_) => unreachable!(),
        PrattError::UnexpectedPostfix(_) => unreachable!(),
    }
}

// a..b
// a..<b
// a..=b
// ..=b
// ..<a
// ..a
// ...
// From this
#[derive(Clone, Debug)]
pub enum ExpressionStream {
    Prefix(ValkyriePrefix),
    Postfix(ValkyrieSuffix),
    Infix(ValkyrieInfix),
    Term(ValkyrieExpression),
    Group(Vec<ExpressionStream>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ValkyrieExpression {
    Placeholder,
    Prefix(Box<ValkyrieUnary>),
    Binary(Box<ValkyrieBinary>),
    Suffix(Box<ValkyrieUnary>),
    Number(Box<NumberLiteralNode>),
    Symbol(Box<NamepathNode>),
    String(Box<StringLiteralNode>),
}

#[derive(Clone, Debug, Eq)]
pub struct ValkyrieUnary {
    pub operator: ValkyrieOperator,
    pub body: ValkyrieExpression,
    pub range: Range<usize>,
}

impl PartialEq for ValkyrieUnary {
    fn eq(&self, other: &Self) -> bool {
        self.operator.eq(&other.operator) && self.body.eq(&other.body)
    }
}
impl PartialEq for ValkyrieBinary {
    fn eq(&self, other: &Self) -> bool {
        self.operator.eq(&other.operator) && self.lhs.eq(&other.lhs) && self.rhs.eq(&other.rhs)
    }
}
#[derive(Clone, Debug, Eq)]
pub struct ValkyrieBinary {
    pub operator: ValkyrieOperator,
    pub lhs: ValkyrieExpression,
    pub rhs: ValkyrieExpression,
    pub range: Range<usize>,
}

impl ThisParser for ValkyrieOperatorKind {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::Operator(self.to_string())
    }
}

impl ValkyrieExpression {
    pub fn binary(o: ValkyrieOperator, lhs: ValkyrieExpression, rhs: ValkyrieExpression) -> ValkyrieExpression {
        let mut out = ValkyrieExpression::Binary(Box::new(ValkyrieBinary { operator: o, lhs, rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn prefix(o: ValkyrieOperator, rhs: ValkyrieExpression) -> ValkyrieExpression {
        let mut out = ValkyrieExpression::Prefix(Box::new(ValkyrieUnary { operator: o, body: rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn suffix(o: ValkyrieOperator, rhs: ValkyrieExpression) -> ValkyrieExpression {
        let mut out = ValkyrieExpression::Suffix(Box::new(ValkyrieUnary { operator: o, body: rhs, range: Default::default() }));
        out.update_range();
        out
    }
    pub fn get_range(&self) -> Range<usize> {
        match self {
            ValkyrieExpression::Placeholder => unreachable!("Placeholder expressions should not be called"),
            ValkyrieExpression::Prefix(u) => u.range.clone(),
            ValkyrieExpression::Binary(b) => b.range.clone(),
            ValkyrieExpression::Suffix(u) => u.range.clone(),
            ValkyrieExpression::Number(u) => u.get_range(),
            ValkyrieExpression::Symbol(u) => u.get_range(),
            ValkyrieExpression::String(u) => u.range.clone(),
        }
    }
    pub fn update_range(&mut self) {
        match self {
            ValkyrieExpression::Prefix(u) => {
                let start = u.operator.range.start;
                let end = u.body.get_range().end;
                u.range = start..end;
            }
            ValkyrieExpression::Binary(b) => {
                let start = b.lhs.get_range().start;
                let end = b.rhs.get_range().end;
                b.range = start..end;
            }
            ValkyrieExpression::Suffix(u) => {
                let start = u.body.get_range().start;
                let end = u.operator.range.end;
                u.range = start..end;
            }
            _ => {}
        }
    }
}

impl<I> PrattParser<I> for ExpressionResolver
where
    I: Iterator<Item = ExpressionStream>,
{
    type Error = StopBecause;
    type Input = ExpressionStream;
    type Output = ValkyrieExpression;

    // Query information about an operator (Affix, Precedence, Associativity)
    fn query(&mut self, tree: &ExpressionStream) -> Result<Affix, StopBecause> {
        let affix = match tree {
            ExpressionStream::Infix(o) => Affix::Infix(o.precedence(), o.associativity()),
            ExpressionStream::Postfix(o) => Affix::Postfix(o.precedence()),
            ExpressionStream::Prefix(o) => Affix::Prefix(o.precedence()),
            ExpressionStream::Group(_) => Affix::Nilfix,
            ExpressionStream::Term(_) => Affix::Nilfix,
        };
        Ok(affix)
    }

    // Construct a primary expression, e.g. a number
    fn primary(&mut self, tree: ExpressionStream) -> Result<ValkyrieExpression, StopBecause> {
        match tree {
            ExpressionStream::Term(term) => Ok(term),
            ExpressionStream::Group(group) => match self.parse(&mut group.into_iter()) {
                Ok(o) => Ok(o),
                Err(e) => say_stop_reason(e),
            },
            _ => unreachable!(),
        }
    }

    // Construct a binary infix expression, e.g. 1+1
    fn infix(
        &mut self,
        lhs: ValkyrieExpression,
        tree: ExpressionStream,
        rhs: ValkyrieExpression,
    ) -> Result<ValkyrieExpression, StopBecause> {
        match tree {
            ExpressionStream::Infix(o) => Ok(ValkyrieExpression::binary(o.as_operator(), lhs, rhs)),
            _ => unreachable!(),
        }
    }

    // Construct a unary prefix expression, e.g. !1
    fn prefix(&mut self, tree: ExpressionStream, rhs: ValkyrieExpression) -> Result<ValkyrieExpression, StopBecause> {
        match tree {
            ExpressionStream::Prefix(o) => Ok(ValkyrieExpression::prefix(o.as_operator(), rhs)),
            _ => unreachable!(),
        }
    }

    // Construct a unary postfix expression, e.g. 1?
    fn postfix(&mut self, lhs: ValkyrieExpression, tree: ExpressionStream) -> Result<ValkyrieExpression, StopBecause> {
        match tree {
            ExpressionStream::Postfix(o) => Ok(ValkyrieExpression::suffix(o.as_operator(), lhs)),
            _ => unreachable!(),
        }
    }
}
