use super::*;
use crate::{expression::TypingExpression, utils::parse_expression_body};
use valkyrie_ast::{CallNode, ExpressionNode, GenericCallTerm};

impl ThisParser for CallNode<GenericCallNode> {
    #[track_caller]
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(3);
        terms.push(Lisp::keyword("call/generic"));
        terms.push(self.base.as_lisp());
        terms.push(self.rest.as_lisp());
        Lisp::Any(terms)
    }
}

impl ThisParser for GenericCallNode {
    /// `::<T> | ⦓T⦔`
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(qwerty_generic).or_else(unicode_generic).end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(Lisp::keyword("generic"));
        // terms.push(self.base.lispify().into());
        for term in &self.terms {
            terms.push(term.term.as_lisp());
        }
        Lisp::Any(terms)
    }
}

impl ThisParser for GenericCallTerm {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, term) = CallTermNode::<IdentifierNode, TypingExpression>::parse(input)?;
        state.finish(GenericCallTerm { term: term.map_value(|s| s.wrapper) })
    }

    fn as_lisp(&self) -> Lisp {
        self.term.as_lisp()
    }
}

fn qwerty_generic(input: ParseState) -> ParseResult<GenericCallNode> {
    let (state, _) = input.match_optional(parse_name_join)?;
    let pat = BracketPattern::new("<", ">");
    let (state, terms) = pat.consume(state.skip(ignore), ignore, GenericCallTerm::parse)?;
    state.finish(GenericCallNode { terms: terms.body, span: get_span(input, state) })
}

fn unicode_generic(input: ParseState) -> ParseResult<GenericCallNode> {
    let pat = BracketPattern::new("⦓", "⦔");
    let (state, terms) = pat.consume(input, ignore, GenericCallTerm::parse)?;
    state.finish(GenericCallNode { terms: terms.body, span: get_span(input, state) })
}
