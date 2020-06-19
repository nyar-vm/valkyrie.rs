use crate::{expression::TermExpressionNode, helpers::ignore, traits::ThisParser};
use lispify::{Lisp, Lispify};
use std::{
    fmt::{Display, Formatter},
    ops::Range,
    str::FromStr,
};
use valkyrie_ast::{ApplyCallNode, IdentifierNode};
use valkyrie_types::third_party::pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};

impl Display for DotApplyNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Lispify for DotApplyNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(self.base.as_lisp().into());
        terms.push(Lisp::Any(vec![Lisp::keyword("."), self.caller.as_lisp().into()]));
        for term in &self.terms {
            terms.push(term.as_lisp().into());
        }
        Lisp::Any(terms)
    }
}

impl FromStr for DotApplyNode {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, DotApplyNode::parse)
    }
}

// ZeroBytePattern::new(&[("⍚", 16), ("⍙", 8), ("⍜", 2)]);
impl DotApplyNode {
    /// ```js
    /// .id(xxx)
    /// ```
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_char('.')?;
        let (state, caller) = state.skip(ignore).match_fn(IdentifierNode::parse)?;
        let (finally, args) = state.skip(ignore).match_fn(ApplyCallNode::parse)?;
        finally.finish(DotApplyNode {
            base: TermExpressionNode::Placeholder,
            caller,
            terms: args.terms,
            range: finally.away_from(input),
        })
    }
}
