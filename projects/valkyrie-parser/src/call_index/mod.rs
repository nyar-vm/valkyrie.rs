use crate::{expression::ValkyrieExpression, helpers::ignore, traits::ThisParser};
use lispify::{Lisp, Lispify};
use pex::{BracketPair, BracketPattern, ParseResult, ParseState};
use valkyrie_ast::{ViewNode, ViewRangeNode, ViewTermNode};

impl ThisParser for ViewNode<ValkyrieExpression> {
    /// `[` ~ `]` | `[` [term](ViewTermNode::parse) ( ~ `,` ~ [term](ViewTermNode::parse))* `,`? `]`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, (index0, terms)) = input.begin_choice().or_else(parse_index0).or_else(parse_index1).end_choice()?;
        state.finish(ViewNode {
            index0,
            base: ValkyrieExpression::Placeholder,
            terms: terms.body,
            range: state.away_from(input),
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(Lisp::function("index"));
        terms.push(self.base.lispify().into());
        for term in &self.terms {
            terms.push(term.as_lisp().into());
        }
        Lisp::Any(terms)
    }
}

#[inline]
fn parse_index0(input: ParseState) -> ParseResult<(bool, BracketPair<ViewTermNode<ValkyrieExpression>>)> {
    let pat = BracketPattern::new("[", "]");
    pat.consume(input, ignore, ViewTermNode::parse).map_inner(|s| (true, s))
}
#[inline]
fn parse_index1(input: ParseState) -> ParseResult<(bool, BracketPair<ViewTermNode<ValkyrieExpression>>)> {
    let pat = BracketPattern::new("⁅", "⁆");
    pat.consume(input, ignore, ViewTermNode::parse).map_inner(|s| (false, s))
}

impl ThisParser for ViewTermNode<ValkyrieExpression> {
    /// [range](ViewTermNode::parse_ranged) | [index](ViewTermNode::parse_single)
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(parse_ranged).or_else(parse_single).end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            ViewTermNode::Index(v) => v.lispify(),
            ViewTermNode::Range(v) => v.as_lisp(),
        }
    }
}

impl ThisParser for ViewRangeNode<ValkyrieExpression> {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(4);
        terms.push(Lisp::function("range").into());
        match &self.start {
            None => terms.push(Lisp::keyword("nil")),
            Some(s) => terms.push(s.lispify().into()),
        }
        match &self.end {
            None => terms.push(Lisp::keyword("nil")),
            Some(s) => terms.push(s.lispify().into()),
        }
        match &self.step {
            None => terms.push(Lisp::keyword("nil")),
            Some(s) => terms.push(s.lispify().into()),
        }
        Lisp::Any(terms)
    }
}

/// [start](ValkyrieExpression::parse)? ~ `:` ~ [end](ValkyrieExpression::parse)? (~ `:` ~ [step](ValkyrieExpression::parse)?)?
pub fn parse_ranged(input: ParseState) -> ParseResult<ViewTermNode<ValkyrieExpression>> {
    let (state, start) = input.match_optional(ValkyrieExpression::parse)?;
    let (state, _) = state.skip(ignore).match_char(':')?;
    let (state, end) = state.skip(ignore).match_optional(ValkyrieExpression::parse)?;
    let (state, step) = state.match_optional(maybe_step)?;
    state.finish(ViewTermNode::ranged(start, end, step.flatten(), state.away_from(input)))
}
/// - [term](ValkyrieExpression::parse)
pub fn parse_single(input: ParseState) -> ParseResult<ViewTermNode<ValkyrieExpression>> {
    let (state, term) = ValkyrieExpression::parse(input)?;
    state.finish(ViewTermNode::indexed(term))
}
/// `~ : ~ step?`
fn maybe_step(input: ParseState) -> ParseResult<Option<ValkyrieExpression>> {
    let (state, _) = input.skip(ignore).match_char(':')?;
    let (state, term) = state.skip(ignore).match_optional(ValkyrieExpression::parse)?;
    state.finish(term)
}
