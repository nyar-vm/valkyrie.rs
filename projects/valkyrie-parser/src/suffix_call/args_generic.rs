use super::*;

impl ThisParser for GenericArgument {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, terms) = input
            .begin_choice()
            .choose(|s| {
                let (state, _) = parse_name_join(s)?;
                let pat = BracketPattern::new("(", ")");
                pat.consume(state.skip(ignore), ignore, GenericArgumentTerm::parse)
            })
            .choose(|s| {
                let pat = BracketPattern::new("<", ">");
                pat.consume(s, ignore, GenericArgumentTerm::parse)
            })
            .choose(|s| {
                let pat = BracketPattern::new("⦓", "⦔");
                pat.consume(s, ignore, GenericArgumentTerm::parse)
            })
            .end_choice()?;
        state.finish(GenericArgument { terms: terms.body, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(self.terms.len() + 2);
        lisp += Lisp::keyword("define/generic");
        for term in &self.terms {
            lisp += term.lispify();
        }
        lisp
    }
}

impl ThisParser for GenericArgumentTerm {
    fn parse(input: ParseState) -> ParseResult<Self> {
        ArgumentTermNode::parse(input).map_inner(|term| GenericArgumentTerm { term })
    }

    fn lispify(&self) -> Lisp {
        self.term.lispify()
    }
}
