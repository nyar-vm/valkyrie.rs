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
            terms += theme.keyword(m.name.clone());
            terms += " ";
        }
        terms += theme.keyword(self.r#type.as_str());
        terms += " ";
        terms += self.namepath.pretty(theme);
        if let Some(gen) = &self.generic {
            terms += gen.pretty(theme);
        }
        terms += self.arguments.pretty(theme);
        if let Some(ret) = &self.r#return {
            terms += ": ";
            terms += ret.returns.pretty(theme);
        }
        terms += self.body.pretty(theme);
        terms.into()
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
        terms += " ";
        terms += "{";
        terms += PrettyTree::Hardline;
        terms += theme.intersperse(&self.terms, PrettyTree::Hardline.indent(4));
        terms += PrettyTree::Hardline;
        terms += "}";
        terms.into()
    }
}
#[cfg(feature = "pretty-print")]
impl<K: PrettyPrint, V: PrettyPrint, D: PrettyPrint> PrettyPrint for ArgumentTermNode<K, V, D> {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms += self.key.pretty(theme);
        if let Some(value) = &self.value {
            terms += ": ";
            terms += value.pretty(theme);
        }
        if let Some(default) = &self.default {
            terms += " = ";
            terms += default.pretty(theme);
        }
        terms.into()
    }
}
