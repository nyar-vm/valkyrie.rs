use super::*;
use pex::StopBecause;
use valkyrie_ast::{CallNode, MatchKind, MatchStatement, PatternBlock};

impl ThisParser for CallNode<MatchStatement> {
    #[track_caller]
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for MatchStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, kind) = MatchKind::parse(input)?;
        let (state, pats) = state.skip(ignore).match_fn(PatternBlock::parse)?;

        state.finish(Self { kind, patterns: pats, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
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

    fn as_lisp(&self) -> Lisp {
        Lisp::keyword(self.as_str())
    }
}
