use super::*;

#[track_caller]
pub fn parse_repl(s: &str) -> Vec<ValkyrieREPL> {
    let input = ParseState::new(s);
    let (state, repl) = match input.match_repeats(ValkyrieREPL::parse) {
        ParseResult::Pending(s, v) => (s.skip(ignore), v),
        ParseResult::Stop(e) => panic!("Failed to parse REPL: {:?}", e),
    };
    if !state.residual.is_empty() {
        panic!("Expect EOF, found:\n{}", state.residual)
    }
    repl
}

#[inline(always)]
fn parse_many(input: ParseState) -> ParseResult<Vec<ValkyrieREPL>> {
    let (state, o) = input.match_repeats(ValkyrieREPL::parse)?;
    state.skip(ignore).finish(o)
}

impl ValkyrieREPL {
    /// - [term](ValkyrieExpression::parse)
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(maybe_expression).end_choice()
    }
}

/// ~ term ~ ;?
fn maybe_expression(input: ParseState) -> ParseResult<ValkyrieREPL> {
    let (state, expr) = input.skip(ignore).match_fn(ValkyrieExpression::parse)?;
    let (state, _) = state.skip(ignore).match_optional(|s| s.match_char(';'))?;
    state.finish(ValkyrieREPL::Expression(Box::new(expr)))
}
