use super::*;

impl ThisParser for CallNode<GenericCallNode> {
    #[track_caller]
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(3);
        lisp += Lisp::keyword("call/parameter");
        lisp += self.base.lispify();
        lisp += self.rest.lispify();
        lisp
    }
}

impl ThisParser for GenericCallNode {
    /// `::<T> | ⦓T⦔`
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().choose(qwerty_generic).choose(unicode_generic).end_choice()
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(self.terms.len() + 2);
        lisp += Lisp::keyword("parameter");
        // terms.push(self.base.lispify().into());
        for term in &self.terms {
            lisp += term.term.lispify();
        }
        lisp
    }
}

impl ThisParser for GenericCallTerm {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, term) = CallTermNode::<IdentifierNode, TypingExpression>::parse(input)?;
        state.finish(GenericCallTerm { term: term.map_value(|s| s.as_normal()) })
    }

    fn lispify(&self) -> Lisp {
        self.term.lispify()
    }
}

fn qwerty_generic(input: ParseState) -> ParseResult<GenericCallNode> {
    let (state, _) = parse_name_join(input)?;
    let pat = BracketPattern::new("(", ")");
    let (state, terms) = pat.consume(state.skip(ignore), ignore, GenericCallTerm::parse)?;
    state.finish(GenericCallNode { terms: terms.body, span: get_span(input, state) })
}

fn unicode_generic(input: ParseState) -> ParseResult<GenericCallNode> {
    let pat = BracketPattern::new("⦓", "⦔");
    let (state, terms) = pat.consume(input, ignore, GenericCallTerm::parse)?;
    state.finish(GenericCallNode { terms: terms.body, span: get_span(input, state) })
}
