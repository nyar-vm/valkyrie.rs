use super::*;
use pex::BracketPattern;
use valkyrie_ast::{PatternBranch, SwitchStatement};

impl ThisParser for SwitchStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("switch")(input)?;
        let pattern = BracketPattern::new("{", "}");
        let (state, terms) = pattern.consume(state.skip(ignore), ignore, PatternBranch::parse)?;
        state.finish(Self { branches: terms.body, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(10);
        terms.push(Lisp::keyword("branches"));
        for branch in &self.branches {
            terms.push(branch.as_lisp());
        }
        Lisp::Any(terms)
    }
}
