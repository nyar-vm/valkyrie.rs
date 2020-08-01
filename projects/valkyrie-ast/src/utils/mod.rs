use crate::{PrettyPrint, PrettyProvider, PrettyTree, StatementNode};
use pretty::{
    termcolor::{Color, ColorSpec},
    DocAllocator,
};

impl<'a> PrettyProvider<'a> {
    pub(crate) fn keyword(&'a self, text: &'static str) -> PrettyTree<'a> {
        let kw = ColorSpec::new().set_fg(Some(Color::Rgb(197, 119, 207))).clone();
        self.text(text).annotate(kw)
    }
    pub(crate) fn string_style(&self) -> ColorSpec {
        ColorSpec::new().set_fg(Some(Color::Rgb(152, 195, 121))).clone()
    }
    pub(crate) fn number_style(&self) -> ColorSpec {
        ColorSpec::new().set_fg(Some(Color::Rgb(206, 153, 100))).clone()
    }
    pub(crate) fn macro_style(&self) -> ColorSpec {
        ColorSpec::new().set_fg(Some(Color::Rgb(87, 182, 194))).clone()
    }
}

impl<'a> PrettyProvider<'a> {
    /// ```vk
    /// # inline style
    /// { ... }
    ///
    /// # block style
    /// {
    ///    ...
    /// }
    /// ```
    pub(crate) fn function_body(&'a self, terms: &[StatementNode]) -> PrettyTree<'a> {
        let inline = self
            .space()
            .append("{")
            .append(self.space())
            .append(self.intersperse(terms.iter().map(|x| x.pretty(self)), self.softline()))
            .append(self.space())
            .append(self.text("}"));
        // 2 or more elements
        let block = self
            .hardline()
            .append(self.text("{"))
            .append(self.hardline())
            .append(self.intersperse(terms.iter().map(|x| x.pretty(self)), self.hardline()).group().indent(4))
            .append(self.hardline())
            .append(self.text("}"));
        block.flat_alt(inline)
    }
}
