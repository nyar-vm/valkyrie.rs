use crate::{expression::TermExpressionNode, helpers::ignore, traits::ThisParser};
use lispify::{Lisp, Lispify};
use valkyrie_ast::{ViewNode, ViewRangeNode, ViewTermNode};
use valkyrie_types::third_party::pex::{BracketPair, BracketPattern, ParseResult, ParseState};

impl ThisParser for ViewNode<TermExpressionNode> {
    /// `[` ~ `]` | `[` [term](ViewTermNode::parse) ( ~ `,` ~ [term](ViewTermNode::parse))* `,`? `]`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, (index0, terms)) = input.begin_choice().or_else(parse_index1).or_else(parse_index0).end_choice()?;
        state.finish(ViewNode {
            index0,
            base: TermExpressionNode::Placeholder,
            terms: terms.body,
            range: state.away_from(input),
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(Lisp::function("index"));
        terms.push(self.base.as_lisp());
        for term in &self.terms {
            terms.push(term.as_lisp());
        }
        Lisp::Any(terms)
    }
}

#[inline]
fn parse_index1(input: ParseState) -> ParseResult<(bool, BracketPair<ViewTermNode<TermExpressionNode>>)> {
    let pat = BracketPattern::new("[", "]");
    pat.consume(input, ignore, ViewTermNode::parse).map_inner(|s| (true, s))
}

#[inline]
fn parse_index0(input: ParseState) -> ParseResult<(bool, BracketPair<ViewTermNode<TermExpressionNode>>)> {
    let pat = BracketPattern::new("⁅", "⁆");
    pat.consume(input, ignore, ViewTermNode::parse).map_inner(|s| (false, s))
}

impl ThisParser for ViewTermNode<TermExpressionNode> {
    /// [range](ViewTermNode::parse_ranged) | [index](ViewTermNode::parse_single)
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(parse_ranged).or_else(parse_single).end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            ViewTermNode::Index(v) => v.as_lisp(),
            ViewTermNode::Range(v) => v.as_lisp(),
        }
    }
}

impl ThisParser for ViewRangeNode<TermExpressionNode> {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(4);
        terms.push(Lisp::function("range").into());
        match &self.start {
            None => terms.push(Lisp::keyword("nil")),
            Some(s) => terms.push(s.as_lisp()),
        }
        match &self.end {
            None => terms.push(Lisp::keyword("nil")),
            Some(s) => terms.push(s.as_lisp()),
        }
        match &self.step {
            None => terms.push(Lisp::keyword("nil")),
            Some(s) => terms.push(s.as_lisp()),
        }
        Lisp::Any(terms)
    }
}

/// [start](TermExpressionNode::parse)? ~ `:` ~ [end](TermExpressionNode::parse)? (~ `:` ~ [step](TermExpressionNode::parse)?)?
pub fn parse_ranged(input: ParseState) -> ParseResult<ViewTermNode<TermExpressionNode>> {
    let (state, start) = input.match_optional(TermExpressionNode::parse)?;
    let (state, _) = state.skip(ignore).match_char(':')?;
    let (state, end) = state.skip(ignore).match_optional(TermExpressionNode::parse)?;
    let (state, step) = state.match_optional(maybe_step)?;
    state.finish(ViewTermNode::ranged(start, end, step.flatten(), state.away_from(input)))
}
/// - [term](TermExpressionNode::parse)
pub fn parse_single(input: ParseState) -> ParseResult<ViewTermNode<TermExpressionNode>> {
    let (state, term) = TermExpressionNode::parse(input)?;
    state.finish(ViewTermNode::indexed(term))
}
/// `~ : ~ step?`
fn maybe_step(input: ParseState) -> ParseResult<Option<TermExpressionNode>> {
    let (state, _) = input.skip(ignore).match_char(':')?;
    let (state, term) = state.skip(ignore).match_optional(TermExpressionNode::parse)?;
    state.finish(term)
}
