use super::*;
use valkyrie_ast::{ArgumentTermNode, GenericArgumentTerm};

impl ThisParser for GenericArgumentNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, terms) = input
            .begin_choice()
            .or_else(|s| {
                let (state, _) = s.match_optional(parse_name_join)?;
                let pat = BracketPattern::new("<", ">");
                pat.consume(state.skip(ignore), ignore, GenericArgumentTerm::parse)
            })
            .or_else(|s| {
                let pat = BracketPattern::new("⦓", "⦔");
                pat.consume(s, ignore, GenericArgumentTerm::parse)
            })
            .end_choice()?;
        state.finish(GenericArgumentNode { terms: terms.body, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(Lisp::keyword("define/generic"));
        // terms.push(self.base.lispify().into());
        for term in &self.terms {
            terms.push(term.as_lisp());
        }
        Lisp::Any(terms)
    }
}

impl ThisParser for GenericArgumentTerm {
    fn parse(input: ParseState) -> ParseResult<Self> {
        ArgumentTermNode::parse(input).map_inner(|term| GenericArgumentTerm { term })
    }

    fn as_lisp(&self) -> Lisp {
        self.term.as_lisp()
    }
}
