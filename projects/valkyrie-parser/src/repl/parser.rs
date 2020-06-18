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

impl ThisParser for ValkyrieREPL {
    /// - [term](TermExpressionNode::parse)
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, expr) = input
            .skip(ignore)
            .begin_choice()
            .or_else(|s| NamespaceDeclareNode::parse(s).map_inner(Into::into))
            .or_else(|s| TermExpressionNode::parse(s).map_inner(Into::into))
            .end_choice()?;
        let (state, _) = state.skip(ignore).match_optional(|s| s.match_char(';'))?;
        state.finish(expr)
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl From<NamespaceDeclareNode> for ValkyrieREPL {
    fn from(value: NamespaceDeclareNode) -> Self {
        ValkyrieREPL::Namespace(Box::new(value))
    }
}

impl From<TermExpressionNode> for ValkyrieREPL {
    fn from(value: TermExpressionNode) -> Self {
        ValkyrieREPL::Expression(Box::new(value))
    }
}
