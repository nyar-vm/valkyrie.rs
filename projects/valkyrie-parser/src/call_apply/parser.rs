use super::*;
use crate::traits::ThisParser;
use valkyrie_ast::PairNode;

impl FromStr for ValkyrieApply {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, ValkyrieApply::parse)
    }
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
