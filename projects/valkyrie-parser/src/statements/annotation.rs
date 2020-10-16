use super::*;
use valkyrie_ast::{AnnotationKind, AnnotationList, AnnotationNode, AnnotationPathNode, AnnotationTerm};

impl ThisParser for AnnotationKind {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .choose(|s| s.match_str("@@").map_inner(|_| AnnotationKind::Environment))
            .choose(|s| s.match_str("@!").map_inner(|_| AnnotationKind::NonCapture))
            .choose(|s| s.match_str("@").map_inner(|_| AnnotationKind::Normal))
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for AnnotationNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, kind) = AnnotationKind::parse(input)?;
        let (state, term) = AnnotationTerm::parse(state.skip(ignore))?;

        state.finish(AnnotationNode { kind, term, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        AnnotationList::from(self.clone()).as_lisp()
    }
}

impl ThisParser for AnnotationTerm {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, name) = AnnotationPathNode::parse(input)?;

        state.finish(AnnotationTerm { path: name, arguments: Default::default(), collects: Default::default() })
    }

    fn as_lisp(&self) -> Lisp {
        self.path.as_lisp()
    }
}

impl ThisParser for AnnotationPathNode {
    /// `a::b::c.d.e.f`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, path) = input.match_fn(NamePathNode::parse)?;
        let (state, names) = state.match_repeats(pare_dot_id)?;
        state.finish(AnnotationPathNode::new(path, names, get_span(input, state)))
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::symbol(self.to_string())
    }
}

/// ~ . ~ id
fn pare_dot_id(input: ParseState) -> ParseResult<IdentifierNode> {
    let (state, _) = input.skip(ignore).match_char('.')?;
    let (state, id) = state.skip(ignore).match_fn(IdentifierNode::parse)?;
    state.finish(id)
}

impl ThisParser for AnnotationList {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, kind) = AnnotationKind::parse(input)?;
        let pattern = BracketPattern::new("[", "]");
        let (state, terms) = pattern.consume(state.skip(ignore), ignore, AnnotationTerm::parse)?;
        state.finish(AnnotationList { kind, terms: terms.body, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(10);
        lisp += Lisp::keyword("annotation/list");
        for term in &self.terms {
            lisp += term.as_lisp();
        }
        lisp
    }
}
