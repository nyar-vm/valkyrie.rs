use super::*;

impl ThisParser for DocumentationNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().choose(parse_one_line).choose(parse_multi_line).end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()

        // Lisp::String(Box::new(ListString { text: "#?".to_string(), unit: self.documentation.clone() }))
    }
}

/// `#? ...`
pub fn parse_one_line(input: ParseState) -> ParseResult<DocumentationNode> {
    let (state, line) = CommentLine::new("#?")(input)?;
    let start = line.head.start_offset() as u32;
    let end = line.body.end_offset() as u32;
    state.finish(DocumentationNode { documentation: line.body.as_string(), span: start..end })
}

/// `#? ...`
pub fn parse_multi_line(input: ParseState) -> ParseResult<DocumentationNode> {
    StopBecause::custom_error("Not implemented yet", input.start_offset, input.start_offset)?
}
