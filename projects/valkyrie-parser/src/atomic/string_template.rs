use super::*;

use valkyrie_ast::StringTemplateNode;

impl ThisParser for StringLiteralNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, unit) = input.match_optional(IdentifierNode::parse)?;
        let (state, pair) = state
            .begin_choice()
            .or_else(|s| quotation_pair_nested(s, '\''))
            .or_else(|s| quotation_pair_nested(s, '"'))
            .or_else(|s| quotation_pair(s, '«', '»'))
            .end_choice()?;

        state.finish(StringLiteralNode { value: pair.body.as_string(), unit, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        ListString { text: self.value.to_owned(), unit: self.unit.clone().map(|u| u.name).unwrap_or_default() }.into()
    }
}

impl ThisParser for StringTemplateNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}
