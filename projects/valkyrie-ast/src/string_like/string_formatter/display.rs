use super::*;

impl Debug for StringFormatterTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Text { unescaped } => {
                f.debug_struct("Text").field("text", &unescaped.text).field("span", &unescaped.span).finish()
            }
            Self::Simple { argument, formatter } => {
                let w = &mut f.debug_struct("Slot");
                w.field("argument", argument);
                if let Some(formatter) = formatter {
                    w.field("formatter", &formatter.text);
                }
                w.finish()
            }
            Self::Complex { argument, formatters } => {
                let w = &mut f.debug_struct("Slot");
                w.field("argument", argument);
                if !formatters.is_empty() {
                    w.field("formatters", formatters);
                }
                w.finish()
            }
        }
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for StringFormatterTerm {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        todo!()
    }
}
