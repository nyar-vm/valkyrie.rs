use super::*;
use lispify::Lispify;

impl ThisParser for FlagsDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("flags")(input)?;
        let (state, id) = state.skip(ignore).match_fn(NamePathNode::parse)?;
        let (state, stmt) = parse_statement_block(state.skip(ignore), enum_statements)?;

        state.finish(FlagsDeclaration {
            documentation: Default::default(),
            name: id,
            modifiers: vec![],
            layout: None,
            implements: vec![],
            body: stmt,
            span: get_span(input, state),
        })
    }
}
