use crate::IdentifierNode;
use pretty::{
    termcolor::{Buffer, Color, ColorSpec},
    Arena, DocAllocator, DocBuilder, Pretty,
};
use std::{
    borrow::Cow,
    ops::{Deref, Range},
    string::String,
};

pub type PrettyTree<'a> = DocBuilder<'a, Arena<'a, ColorSpec>, ColorSpec>;

pub trait ValkyrieNode {
    fn get_range(&self) -> Range<u32>;
    // fn mut_range(&mut self) -> &mut Range<u32>;
}

pub struct PrettyProvider<'a> {
    arena: Arena<'a, ColorSpec>,
}

impl<'a> Deref for PrettyProvider<'a> {
    type Target = Arena<'a, ColorSpec>;

    fn deref(&self) -> &Self::Target {
        &self.arena
    }
}

impl<'a> PrettyProvider<'a> {
    pub fn new() -> Self {
        PrettyProvider { arena: Arena::new() }
    }
}

pub trait PrettyPrint {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a>;
    fn pretty_string(&self, width: usize) -> String {
        let arena = PrettyProvider::new();
        let mut buffer = Buffer::ansi();
        if let Err(e) = self.build(&arena).render(width, &mut buffer) {
            return format!("Error: {}", e);
        }
        unsafe { String::from_utf8_unchecked(buffer.into_inner()) }
    }
    fn pretty_print(&self, width: usize) {
        let arena = PrettyProvider::new();
        let mut buffer = Buffer::ansi();
        match self.build(&arena).render_colored(width, &mut buffer) {
            Ok(_) => {
                println!("{}", unsafe { String::from_utf8_unchecked(buffer.into_inner()) });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

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

impl<'a> PrettyProvider<'a> {
    pub(crate) fn join<T, S>(&'a self, terms: &[T], joint: S) -> PrettyTree<'a>
    where
        T: PrettyPrint,
        S: Pretty<'a, Arena<'a, ColorSpec>, ColorSpec> + Clone,
    {
        self.intersperse(terms.iter().map(|x| x.build(self)), joint)
    }
}
