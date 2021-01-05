use super::*;

impl ThisParser for CallNode<MatchDotStatement> {
    #[track_caller]
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(5);
        lisp += Lisp::keyword(self.rest.kind.as_str());
        for term in &self.rest.patterns.branches {
            lisp += term.lispify()
        }
        lisp
    }
}

impl ThisParser for MatchDotStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, monadic) = MonadicDotCall::parse(input)?;
        let (state, kind) = MatchKind::parse(state)?;
        let (state, pats) = state.skip(ignore).match_fn(PatternBlock::parse)?;
        state.finish(Self { monadic, kind, patterns: pats, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for MatchKind {
    fn parse(input: ParseState) -> ParseResult<Self> {
        if input.residual.starts_with("match") {
            input.advance("match").finish(Self::Typing)
        }
        else if input.residual.starts_with("catch") {
            input.advance("catch").finish(Self::Effect)
        }
        else {
            StopBecause::must_be("KW_MATCH", input.start_offset)?
        }
    }

    fn lispify(&self) -> Lisp {
        Lisp::keyword(self.as_str())
    }
}
