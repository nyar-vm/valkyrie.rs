use super::*;
use crate::{helpers::parse_name_join, traits::ThisParser};
use valkyrie_ast::{GenericArgumentNode, GenericCall};

impl FromStr for ValkyrieApply {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieApply::parse)
    }
}

impl<E> ThisParser for GenericArgumentNode<E> {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for GenericCall<ValkyrieExpression> {
    /// `::<T> | ⦓T⦔`
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .or_else(qwerty_generic)
            // .or_else(unicode_generic)
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(Lisp::function("generic"));
        // terms.push(self.base.lispify().into());
        for term in &self.terms {
            terms.push(term.as_lisp().into());
        }
        Lisp::Any(terms)
    }
}

fn qwerty_generic(input: ParseState) -> ParseResult<GenericCall<ValkyrieExpression>> {
    let pat = BracketPattern::new("<", ">");
    let (state, _) = input.match_optional(parse_name_join)?;
    let (state, terms) = pat.consume(state.skip(ignore), ignore, TableTermNode::parse)?;
    state.finish(GenericCall { terms: terms.body, range: state.away_from(input) })
}

fn unicode_generic(input: ParseState) -> ParseResult<GenericCall<ValkyrieExpression>> {
    let pat = BracketPattern::new("⦓", "⦔");
    let (state, terms) = pat.consume(input, ignore, TableTermNode::parse)?;
    state.finish(GenericCall { terms: terms.body, range: state.away_from(input) })
}

impl ThisParser for ValkyrieApply {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("(", ")");
        let (state, terms) = pat.consume(input, ignore, TableTermNode::parse)?;
        state.finish(ValkyrieApply { base: ValkyrieExpression::Placeholder, terms: terms.body, range: state.away_from(input) })
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
