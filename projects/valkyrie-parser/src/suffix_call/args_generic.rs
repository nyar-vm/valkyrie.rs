use super::*;

impl ThisParser for GenericArgument {
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
        state.finish(GenericArgument { terms: terms.body, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(self.terms.len() + 2);
        lisp += Lisp::keyword("define/generic");
        for term in &self.terms {
            lisp += term.as_lisp();
        }
        lisp
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
