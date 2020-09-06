use super::*;
use valkyrie_ast::CollectsNode;

impl ThisParser for NewConstructNode {
    /// ```vk
    /// let a = new mod1 mod2 module.rest.Class<G>(args) {
    ///     [a]: 2,
    ///     Size: Math.PI,
    ///     ['C',4]: "Middle C",
    ///     Pair(0, 2),
    ///     term,
    ///     other,
    /// }
    /// ```
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("new")?;
        let (state, mut modifiers) = state.match_repeats(|s| s.skip(ignore).match_fn(IdentifierNode::parse))?;
        let last = modifiers.pop();
        let mut names = match last {
            Some(s) => vec![s],
            None => StopBecause::custom_error("Expected a name for the new structure", input.start_offset, state.start_offset)?,
        };
        let (state, name_rest) = state.match_repeats(|s| {
            let (state, _) = s.skip(ignore).match_fn(parse_name_join_dot)?;
            IdentifierNode::parse(state)
        })?;
        names.extend(name_rest);
        let namepath = NamePathNode::new(names);
        let (state, generic) = state.skip(ignore).match_optional(GenericCallNode::parse)?;
        let (state, arguments) = state.skip(ignore).match_optional(ApplyCallNode::parse)?;
        let (finally, collects) = state.skip(ignore).match_optional(CollectsNode::parse)?;
        finally.finish(NewConstructNode {
            modifiers,
            namepath,
            generic: generic.unwrap_or_default(),
            arguments: arguments.unwrap_or_default(),
            body: collects.unwrap_or_default(),
            span: get_span(input, finally),
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.body.terms.len() + 3);
        terms.push(Lisp::keyword("new"));
        terms.push(self.generic.as_lisp());
        terms.push(self.arguments.as_lisp());
        for term in &self.body.terms {
            terms.push(term.as_lisp())
        }
        Lisp::Any(terms)
    }
}

impl ThisParser for CollectsNode {
    /// ```vk
    /// {
    ///     [a]: 2,
    ///     Size: Math.PI,
    ///     ['C',4]: "Middle C",
    ///     Pair(0, 2),
    ///     term,
    ///     other,
    /// }
    /// ```
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("{", "}");
        let (state, terms) = pat.consume(input, ignore, TableTermNode::parse)?;
        state.finish(CollectsNode { terms: terms.body, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}
