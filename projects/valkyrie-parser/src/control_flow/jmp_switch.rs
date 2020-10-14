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
        let mut lisp = Lisp::new(10);
        lisp += Lisp::keyword("branches");
        for branch in &self.branches {
            lisp += branch.as_lisp();
        }
        lisp
    }
}
