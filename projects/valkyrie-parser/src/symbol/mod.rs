use regex::Regex;
use std::sync::LazyLock;

use crate::expression::ValkyrieExpression;
use pex::{ParseResult, ParseState};

use valkyrie_ast::NamePathNode;

use crate::{helpers::ignore, traits::ThisParser};
use lispify::{Lisp, LispSymbol};
use valkyrie_ast::{IdentifierNode, MacroPathNode};

impl ThisParser for MacroPathNode {
    /// `a::b::c.d.e.f`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, path) = input.match_fn(NamePathNode::parse)?;
        let (state, names) = state.match_repeats(pare_dot_id)?;
        state.finish(MacroPathNode::new(path, names, state.away_from(input)))
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

/// ~ . ~ id
fn pare_dot_id(input: ParseState) -> ParseResult<IdentifierNode> {
    let (state, _) = input.skip(ignore).match_char('.')?;
    let (state, id) = state.skip(ignore).match_fn(IdentifierNode::parse)?;
    state.finish(id)
}

pub static IDENTIFIER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?x)(
      [∞∅]
    | (?P<regular>(?:\p{XID_Start}|_)\p{XID_Continue}*)
    | `(?P<escaped>(?:\\.|[^`])*)`
)",
    )
    .unwrap()
});

impl ThisParser for IdentifierNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&IDENTIFIER, "IDENTIFIER")?;
        let id = IdentifierNode::new(m.as_str(), m.range());
        state.finish(id)
    }

    fn as_lisp(&self) -> Lisp {
        LispSymbol { name: self.name.clone(), path: vec![] }.into()
    }
}

impl ThisParser for NamePathNode {
    /// `id (~ :: ~ b)*`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let mut names = Vec::new();
        let (state, id) = input.match_fn(IdentifierNode::parse)?;
        names.push(id);
        let (state, _) = state.match_repeats(|s| pare_colon_id(s, &mut names))?;
        state.finish(NamePathNode::new(names, state.away_from(input)))
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = self.names.iter().map(|s| s.name.clone());
        let first = terms.next().unwrap_or_default();
        LispSymbol { name: first, path: terms.collect() }.into()
    }
}

fn pare_colon_id<'i>(input: ParseState<'i>, names: &mut Vec<IdentifierNode>) -> ParseResult<'i, ()> {
    let (state, _) = input
        .begin_choice()
        .or_else(|s| s.match_str("::").map_inner(|_| ()))
        .or_else(|s| s.match_char('∷').map_inner(|_| ()))
        .end_choice()?;
    let (state, id) = state.match_fn(|s| IdentifierNode::parse(s))?;
    names.push(id);
    state.finish(())
}

/// These names cannot be used as function names
pub const KEYWORDS: [&str; 3] = ["true", "false", "null"];

impl From<NamePathNode> for ValkyrieExpression {
    fn from(value: NamePathNode) -> Self {
        ValkyrieExpression::Symbol(Box::new(value))
    }
}
