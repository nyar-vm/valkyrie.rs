use super::*;
use crate::traits::ThisParser;
use valkyrie_ast::NamespaceDeclareNode;

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

impl ValkyrieREPL {
    /// - [term](ValkyrieExpression::parse)
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(maybe_expression).end_choice()
    }
}

/// ~ term ~ ;?
fn maybe_expression(input: ParseState) -> ParseResult<ValkyrieREPL> {
    let (state, expr) = input
        .skip(ignore)
        .begin_choice()
        .or_else(|s| NamespaceDeclareNode::parse(s).map_inner(Into::into))
        .or_else(|s| ValkyrieExpression::parse(s).map_inner(Into::into))
        .end_choice()?;
    let (state, _) = state.skip(ignore).match_optional(|s| s.match_char(';'))?;
    state.finish(expr)
}

impl From<NamespaceDeclareNode> for ValkyrieREPL {
    fn from(value: NamespaceDeclareNode) -> Self {
        ValkyrieREPL::Namespace(Box::new(value))
    }
}

impl From<ValkyrieExpression> for ValkyrieREPL {
    fn from(value: ValkyrieExpression) -> Self {
        ValkyrieREPL::Expression(Box::new(value))
    }
}
