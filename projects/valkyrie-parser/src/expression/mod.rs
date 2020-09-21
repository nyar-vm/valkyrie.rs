mod expression_kind;
mod infix;
mod pattern_match;
mod prefix;
mod suffix;

use crate::{
    helpers::{ignore, parse_comma, parse_when},
    table::TupleNode,
    traits::ThisParser,
    utils::{get_span, parse_expression_body, parse_expression_node, parse_modifiers_lookahead},
};
use lispify::Lisp;
use pex::{helpers::str, BracketPattern, ParseResult, ParseState, Regex, StopBecause};
use pratt::{Affix, Associativity, PrattError, PrattParser, Precedence};
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
    sync::LazyLock,
};
use valkyrie_ast::{
    ApplyCallNode, ApplyDotNode, ArgumentKeyNode, ExpressionBody, ExpressionContext, ExpressionNode, GenericCallNode,
    InfixNode, LambdaCallNode, LambdaDotNode, NamePathNode, NewConstructNode, NumberLiteralNode, OperatorNode, PatternBranch,
    PatternCaseNode, PatternCondition, PatternElseNode, PatternExpressionNode, PatternGuard, PatternStatements,
    PatternTypeNode, PatternWhenNode, PostfixCallPart, PostfixNode, PrefixNode, StatementNode, StringLiteralNode,
    SubscriptNode, TableNode, TypingExpression, ValkyrieOperator,
};

/// A resolver
#[derive(Default)]
pub struct ExpressionResolver {}

#[derive(Clone, Debug)]
pub enum ExpressionStream {
    Prefix(ValkyriePrefix),
    Postfix(ValkyrieSuffix),
    Infix(ValkyrieInfix),
    Term(ExpressionBody),
    Group(Vec<ExpressionStream>),
}

#[derive(Clone)]
pub struct ValkyrieInfix {
    pub normalized: String,
    pub span: Range<u32>,
}

#[derive(Clone)]
pub struct ValkyriePrefix {
    pub normalized: String,
    pub span: Range<u32>,
}

#[derive(Clone)]
pub struct ValkyrieSuffix {
    pub normalized: String,
    pub span: Range<u32>,
}

impl Debug for ValkyrieInfix {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Infix({}, {:?})", self.as_operator().kind.as_str(), self.span)
    }
}

impl ValkyrieInfix {
    pub fn new<S: AsRef<str>>(infix: S, range: Range<u32>) -> ValkyrieInfix {
        let text = infix.as_ref();
        let mut normalized = String::with_capacity(text.len());
        for c in text.chars() {
            match c {
                ' ' => continue,
                '∈' | '∊' => normalized.push_str("in"),
                '∉' => normalized.push_str("notin"),
                '⊑' => normalized.push_str("is"),
                '⋢' => normalized.push_str("isnot"),
                '≖' => normalized.push_str("=="),
                '≠' => normalized.push_str("!="),
                '≡' => normalized.push_str("==="),
                '≢' => normalized.push_str("=!="),
                '≫' => normalized.push_str(">>"),
                '≪' => normalized.push_str("<<"),
                '⋙' => normalized.push_str(">>>"),
                '⋘' => normalized.push_str("<<<"),
                _ => normalized.push(c),
            }
        }
        ValkyrieInfix { normalized, span: range }
    }
    pub fn precedence(&self) -> Precedence {
        Precedence(self.as_operator().kind.precedence())
    }
    pub fn associativity(&self) -> Associativity {
        match self.normalized.as_str() {
            "^" => Associativity::Right,
            _ => Associativity::Left,
        }
    }
    pub fn as_operator(&self) -> OperatorNode {
        let kind = match self.normalized.as_str() {
            "++" => ValkyrieOperator::Concat,
            "+" => ValkyrieOperator::Plus,
            "-" => ValkyrieOperator::Minus,
            "*" => ValkyrieOperator::Multiply,
            "/" => ValkyrieOperator::Divide,
            "^" => ValkyrieOperator::Power,
            "|" => ValkyrieOperator::BitOr,
            "&" => ValkyrieOperator::BitAnd,
            ">" => ValkyrieOperator::Greater,
            ">>" => ValkyrieOperator::MuchGreater,
            ">>>" => ValkyrieOperator::VeryMuchGreater,
            "<" => ValkyrieOperator::Less,
            "<<" => ValkyrieOperator::MuchLess,
            "<<<" => ValkyrieOperator::VeryMuchLess,
            "==" => ValkyrieOperator::Equal(true),
            "!=" => ValkyrieOperator::Equal(false),
            "===" => ValkyrieOperator::StrictlyEqual(true),
            "!==" | "=!=" => ValkyrieOperator::StrictlyEqual(false),
            "in" => ValkyrieOperator::Belongs(true),
            "notin" => ValkyrieOperator::Belongs(false),
            "is" => ValkyrieOperator::IsA(true),
            "isnot" => ValkyrieOperator::IsA(false),
            "+=" => ValkyrieOperator::PlusAssign,
            "=" => ValkyrieOperator::Assign,
            _ => unreachable!("Unknown operator: {}", self.normalized),
        };
        OperatorNode::new(kind, self.span.clone())
    }
}
impl ThisParser for TypingExpression {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, out) = parse_expression_node(input, ExpressionContext::in_type())?;
        state.finish(TypingExpression { body: out.body, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl ExpressionResolver {
    pub fn resolve(&self, stream: Vec<ExpressionStream>) -> Result<ExpressionBody, StopBecause> {
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
        PrattError::EmptyInput => Err(StopBecause::ShouldNotBe { message: "EOF", position: 0 }),
        PrattError::UnexpectedNilfix(_) => unreachable!(),
        PrattError::UnexpectedPrefix(_) => unreachable!(),
        PrattError::UnexpectedInfix(_) => unreachable!(),
        PrattError::UnexpectedPostfix(_) => unreachable!(),
    }
}

impl<I> PrattParser<I> for ExpressionResolver
where
    I: Iterator<Item = ExpressionStream>,
{
    type Error = StopBecause;
    type Input = ExpressionStream;
    type Output = ExpressionBody;

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
    fn primary(&mut self, tree: ExpressionStream) -> Result<ExpressionBody, StopBecause> {
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
        lhs: ExpressionBody,
        tree: ExpressionStream,
        rhs: ExpressionBody,
    ) -> Result<ExpressionBody, StopBecause> {
        match tree {
            ExpressionStream::Infix(o) => Ok(ExpressionBody::binary(o.as_operator(), lhs, rhs)),
            _ => unreachable!(),
        }
    }

    // Construct a unary prefix expression, e.g. !1
    fn prefix(&mut self, tree: ExpressionStream, rhs: ExpressionBody) -> Result<ExpressionBody, StopBecause> {
        match tree {
            ExpressionStream::Prefix(o) => Ok(ExpressionBody::prefix(o.as_operator(), rhs)),
            _ => unreachable!(),
        }
    }

    // Construct a unary postfix expression, e.g. 1?
    fn postfix(&mut self, lhs: ExpressionBody, tree: ExpressionStream) -> Result<ExpressionBody, StopBecause> {
        match tree {
            ExpressionStream::Postfix(o) => Ok(ExpressionBody::suffix(o.as_operator(), lhs)),
            _ => unreachable!(),
        }
    }
}
