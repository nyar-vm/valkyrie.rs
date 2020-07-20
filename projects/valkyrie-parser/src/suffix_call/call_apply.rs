use super::*;
use valkyrie_ast::ArgumentTermNode;

impl ThisParser for GenericArgumentNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, terms) = input
            .begin_choice()
            .or_else(|s| {
                let (state, _) = s.match_optional(parse_name_join)?;
                let pat = BracketPattern::new("<", ">");
                pat.consume(state.skip(ignore), ignore, ArgumentTermNode::parse)
            })
            .or_else(|s| {
                let pat = BracketPattern::new("⦓", "⦔");
                pat.consume(s, ignore, ArgumentTermNode::parse)
            })
            .end_choice()?;
        state.finish(GenericArgumentNode { terms: terms.body, range: state.away_from(input) })
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

impl ThisParser for ApplyCallNode<ExpressionBody> {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("(", ")");
        let (state, terms) = pat.consume(input, ignore, ApplyTermNode::parse)?;
        state.finish(ApplyCallNode { base: ExpressionBody::Placeholder, terms: terms.body, range: state.away_from(input) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(Lisp::keyword("apply"));
        terms.push(self.base.as_lisp());
        for term in &self.terms {
            terms.push(term.as_lisp());
        }
        Lisp::Any(terms)
    }
}
