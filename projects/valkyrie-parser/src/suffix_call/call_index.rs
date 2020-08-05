use super::*;
use valkyrie_ast::ExpressionNode;

impl ThisParser for SubscriptNode {
    /// `[` ~ `]` | `[` [term](SubscriptTermNode::parse) ( ~ `,` ~ [term](SubscriptTermNode::parse))* `,`? `]`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, (index0, terms)) = input.begin_choice().or_else(parse_index1).or_else(parse_index0).end_choice()?;
        state.finish(SubscriptNode {
            index0,
            base: ExpressionNode::default(),
            terms: terms.body,
            range: state.away_from(input),
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(Lisp::keyword(self.method()));
        terms.push(self.base.as_lisp());
        for term in &self.terms {
            terms.push(term.as_lisp());
        }
        Lisp::Any(terms)
    }
}

#[inline]
fn parse_index_fake(input: ParseState) -> ParseResult<(bool, BracketPair<SubscriptTermNode>)> {
    let pat = BracketPattern::new("{", "}");
    pat.consume(input, ignore, SubscriptTermNode::parse).map_inner(|s| (false, s))
}

#[inline]
fn parse_index1(input: ParseState) -> ParseResult<(bool, BracketPair<SubscriptTermNode>)> {
    let pat = BracketPattern::new("[", "]");
    pat.consume(input, ignore, SubscriptTermNode::parse).map_inner(|s| (false, s))
}

#[inline]
fn parse_index0(input: ParseState) -> ParseResult<(bool, BracketPair<SubscriptTermNode>)> {
    let pat = BracketPattern::new("⁅", "⁆");
    pat.consume(input, ignore, SubscriptTermNode::parse).map_inner(|s| (true, s))
}

impl ThisParser for SubscriptTermNode {
    /// [range](SubscriptTermNode::parse_ranged) | [index](SubscriptTermNode::parse_single)
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(parse_ranged).or_else(parse_single).end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            SubscriptTermNode::Index(v) => v.as_lisp(),
            SubscriptTermNode::Slice(v) => v.as_lisp(),
        }
    }
}

impl ThisParser for SubscriptSliceNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(4);
        terms.push(Lisp::function("range").into());
        match &self.start {
            None => terms.push(Lisp::keyword("null")),
            Some(s) => terms.push(s.as_lisp()),
        }
        match &self.end {
            None => terms.push(Lisp::keyword("null")),
            Some(s) => terms.push(s.as_lisp()),
        }
        match &self.step {
            None => terms.push(Lisp::keyword("null")),
            Some(s) => terms.push(s.as_lisp()),
        }
        Lisp::Any(terms)
    }
}

/// [start](ExpressionBody::parse)? ~ `:` ~ [end](ExpressionBody::parse)? (~ `:` ~ [step](ExpressionBody::parse)?)?
pub fn parse_ranged(input: ParseState) -> ParseResult<SubscriptTermNode> {
    let (state, start) = input.match_optional(ExpressionNode::parse)?;
    let (state, _) = state.skip(ignore).match_char(':')?;
    let (state, end) = state.skip(ignore).match_optional(ExpressionNode::parse)?;
    let (state, step) = state.match_optional(maybe_step)?;
    state.finish(SubscriptTermNode::ranged(start, end, step.flatten(), state.away_from(input)))
}
/// - [term](ExpressionBody::parse)
pub fn parse_single(input: ParseState) -> ParseResult<SubscriptTermNode> {
    let (state, term) = ExpressionNode::parse(input)?;
    state.finish(SubscriptTermNode::indexed(term))
}

/// `~ : ~ step?`
fn maybe_step(input: ParseState) -> ParseResult<Option<ExpressionNode>> {
    let (state, _) = input.skip(ignore).match_char(':')?;
    let (state, term) = state.skip(ignore).match_optional(ExpressionNode::parse)?;
    state.finish(term)
}
