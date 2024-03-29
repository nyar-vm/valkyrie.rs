use super::*;

impl crate::TextLiteralNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> StringLiteralNode {
        let handler = self.identifier.as_ref().map(|v| v.build(ctx.file));
        let literal = self.text_raw.build();
        StringLiteralNode { literal, handler }
    }
}

impl crate::TextRawNode {
    pub(crate) fn build(&self) -> StringTextNode {
        let mut buffer = String::new();
        if let Some(s) = &self.text_content_1 {
            buffer.push_str(&s.text)
        }
        if let Some(s) = &self.text_content_2 {
            buffer.push_str(&s.text)
        }
        if let Some(s) = &self.text_content_3 {
            buffer.push_str(&s.text)
        }
        if let Some(s) = &self.text_content_4 {
            buffer.push_str(&s.text)
        }
        if let Some(s) = &self.text_content_5 {
            buffer.push_str(&s.text)
        }
        if let Some(s) = &self.text_content_6 {
            buffer.push_str(&s.text)
        }
        StringTextNode { text: buffer, span: self.span.clone() }
    }
    pub(crate) fn build_id(&self, ctx: &mut ProgramState) -> IdentifierNode {
        IdentifierNode { name: "".to_string(), span: ctx.file.with_range(self.get_range()) }
    }
}
