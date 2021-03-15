use super::*;

impl crate::TextLiteralNode {
    pub fn build(&self, ctx: &ProgramContext) -> StringLiteralNode {
        let handler = self.identifier.as_ref().map(|v| v.build(ctx));
        let literal = self.text_raw.build();
        StringLiteralNode { literal, handler }
    }
}

impl crate::TextRawNode {
    pub fn build(&self) -> StringTextNode {
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
}
