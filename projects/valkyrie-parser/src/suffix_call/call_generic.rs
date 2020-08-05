use super::*;
use valkyrie_ast::{CallNode, ExpressionNode};

impl ThisParser for CallNode<GenericNode> {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(3);
        terms.push(Lisp::keyword("call/generic"));
        terms.push(self.base.as_lisp());
        terms.push(self.rest.as_lisp());
        Lisp::Any(terms)
    }
}

impl ThisParser for GenericNode {
    /// `::<T> | ⦓T⦔`
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(qwerty_generic).or_else(unicode_generic).end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(Lisp::keyword("generic"));
        // terms.push(self.base.lispify().into());
        for term in &self.terms {
            terms.push(term.as_lisp());
        }
        Lisp::Any(terms)
    }
}

fn qwerty_generic(input: ParseState) -> ParseResult<GenericNode> {
    let pat = BracketPattern::new("<", ">");
    let (state, _) = input.match_optional(parse_name_join)?;
    let (state, terms) = pat.consume(state.skip(ignore), ignore, CallTermPair::parse)?;
    state.finish(GenericNode { base: ExpressionNode::default(), terms: terms.body, range: state.away_from(input) })
}

fn unicode_generic(input: ParseState) -> ParseResult<GenericNode> {
    let pat = BracketPattern::new("⦓", "⦔");
    let (state, terms) = pat.consume(input, ignore, CallTermPair::parse)?;
    state.finish(GenericNode { base: ExpressionNode::default(), terms: terms.body, range: state.away_from(input) })
}
