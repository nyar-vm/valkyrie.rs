use super::*;

impl ExpressionStream {
    /// term (~ infix ~ term)*
    /// 1 + (1 + +3? + 4)
    pub fn parse(state: ParseState) -> ParseResult<Vec<ExpressionStream>> {
        let mut stream = Vec::new();
        let (state, _) = state.match_fn(|s| parse_term(s, &mut stream))?;
        let (state, _) = state.match_repeats(|s| parse_infix_term(s, &mut stream))?;
        state.finish(stream)
    }
}

/// `~ infix ~ term`
#[inline(always)]
fn parse_infix_term<'i>(input: ParseState<'i>, stream: &mut Vec<ExpressionStream>) -> ParseResult<'i, ()> {
    let (state, infix) = input.skip(ignore).match_fn(ValkyrieInfix::parse).map_inner(ExpressionStream::Infix)?;
    stream.push(infix);
    let (state, _) = state.skip(ignore).match_fn(|s| parse_term(s, stream))?;
    state.finish(())
}

/// `( ~ term ~ )`
pub fn parse_group(state: ParseState) -> ParseResult<Vec<ExpressionStream>> {
    let mut group = Vec::with_capacity(4);
    let (state, _) = state.skip(ignore).match_char('(')?;
    let (state, _) = state.skip(ignore).match_fn(|s| parse_pure_term(s, &mut group))?;
    let (state, _) = state.skip(ignore).match_char(')')?;
    // Only join the global stream after all success
    state.finish(group)
}

/// `(~ prefix)* ~ value (~ suffix)*`
fn parse_pure_term<'i>(state: ParseState<'i>, stream: &mut Vec<ExpressionStream>) -> ParseResult<'i, ()> {
    let (state, _) = state.match_repeats(|s| parse_prefix(s, stream))?;
    let (state, _) = parse_expr_value(state, stream)?;
    let (state, _) = state.match_repeats(|s| parse_suffix(s, stream))?;
    state.finish(())
}

#[inline(always)]
fn parse_prefix<'a>(input: ParseState<'a>, stream: &mut Vec<ExpressionStream>) -> ParseResult<'a, ()> {
    let (state, prefix) = input.skip(ignore).match_fn(ValkyriePrefix::parse)?;
    stream.push(ExpressionStream::Prefix(prefix));
    state.finish(())
}

#[inline(always)]
fn parse_suffix<'a>(input: ParseState<'a>, stream: &mut Vec<ExpressionStream>) -> ParseResult<'a, ()> {
    let (state, suffix) = input.skip(ignore).match_fn(ValkyrieSuffix::parse)?;
    stream.push(ExpressionStream::Postfix(suffix));
    state.finish(())
}

#[inline(always)]
fn parse_expr_value<'a>(input: ParseState<'a>, stream: &mut Vec<ExpressionStream>) -> ParseResult<'a, ()> {
    let (state, term) = input
        .skip(ignore)
        .begin_choice()
        .or_else(|s| parse_group(s).map_inner(ExpressionStream::Group))
        .or_else(|s| parse_value(s).map_inner(ExpressionStream::Term))
        .end_choice()?;
    stream.push(term);
    state.finish(())
}
