use crate::{ArgumentTermNode, IdentifierNode, NamePathNode, PrettyPrint, PrettyProvider, PrettyTree, StatementNode};
use pretty::{
    termcolor::{Color, ColorSpec},
    DocAllocator,
};
use std::borrow::Cow;

impl<'a> PrettyProvider<'a> {
    pub(crate) fn keyword<S>(&'a self, text: S) -> PrettyTree<'a>
    where
        S: Into<Cow<'static, str>>,
    {
        let kw = ColorSpec::new().set_fg(Some(Color::Rgb(197, 119, 207))).clone();
        self.text(text.into()).annotate(kw)
    }
    pub(crate) fn identifier<S>(&'a self, text: S) -> PrettyTree<'a>
    where
        S: Into<Cow<'static, str>>,
    {
        self.operator(text)
    }
    pub(crate) fn generic<S>(&'a self, text: S) -> PrettyTree<'a>
    where
        S: Into<Cow<'static, str>>,
    {
        self.text(text.into()).annotate(self.macro_style())
    }
    pub(crate) fn argument<S>(&'a self, text: S) -> PrettyTree<'a>
    where
        S: Into<Cow<'static, str>>,
    {
        let kw = ColorSpec::new().set_fg(Some(Color::Rgb(239, 112, 117))).clone();
        self.text(text.into()).annotate(kw)
    }
    pub(crate) fn operator<S>(&'a self, text: S) -> PrettyTree<'a>
    where
        S: Into<Cow<'static, str>>,
    {
        let kw = ColorSpec::new().set_fg(Some(Color::Rgb(90, 173, 238))).clone();
        self.text(text.into()).annotate(kw)
    }

    pub(crate) fn namepath(&'a self, path: &[IdentifierNode]) -> PrettyTree<'a> {
        self.intersperse(path.iter().map(|x| self.identifier(x.name.clone())), self.text("::"))
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
