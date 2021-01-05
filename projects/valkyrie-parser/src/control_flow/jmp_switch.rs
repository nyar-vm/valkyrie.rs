use super::*;

impl ThisParser for SwitchStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("switch")(input)?;
        let (state, pats) = state.skip(ignore).match_fn(PatternBlock::parse)?;
        state.finish(Self { patterns: pats, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(10);
        lisp += Lisp::keyword("switch");
        for branch in &self.patterns.branches {
            lisp += branch.lispify();
        }
        lisp
    }
}
