use super::*;

impl FunctionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FunctionType::Macro => "macro",
            FunctionType::Micro => "micro",
        }
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for FunctionType {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.keyword(self.as_str())
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for FunctionDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        for m in &self.modifiers {
            terms.push(theme.keyword(m.name.clone()));
            terms.push(theme.space());
        }
        terms.push(theme.keyword(self.r#type.as_str()));
        terms.push(theme.space());
        terms.push(self.namepath.build(theme));
        if let Some(gen) = &self.generic {
            terms.push(gen.build(theme));
        }
        terms.push(self.arguments.build(theme));
        if let Some(ret) = &self.r#return {
            terms.push(theme.text(": "));
            terms.push(ret.returns.build(theme));
        }
        terms.push(self.body.build(theme));
        theme.concat(terms)
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for StatementBlock {
    /// ```vk
    /// # inline style
    /// { ... }
    ///
    /// # block style
    /// {
    ///    ...
    /// }
    /// ```
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(9);
        terms.push(theme.space());
        terms.push(theme.text("{"));
        terms.push(theme.hardline());
        terms.push(theme.intersperse(&self.terms, theme.hardline()).indent(4));
        terms.push(theme.hardline());
        terms.push(theme.text("}"));
        theme.concat(terms)
    }
}
#[cfg(feature = "pretty-print")]
impl<K: PrettyPrint, V: PrettyPrint, D: PrettyPrint> PrettyPrint for ArgumentTermNode<K, V, D> {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms.push(self.key.build(theme));
        if let Some(value) = &self.value {
            terms.push(theme.text(": "));
            terms.push(value.build(theme));
        }
        if let Some(default) = &self.default {
            terms.push(theme.text(" = "));
            terms.push(default.build(theme));
        }
        theme.concat(terms)
    }
}
