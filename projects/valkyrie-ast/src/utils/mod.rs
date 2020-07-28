use crate::PrettyProvider;
use core::fmt::Write;
use indentation::{IndentDisplay, IndentFormatter};
use pretty::termcolor::{Color, ColorSpec};

pub fn comma_terms<T: IndentDisplay>(f: &mut IndentFormatter, input: &[T]) -> core::fmt::Result {
    let mut terms = input.iter();
    match terms.next() {
        Some(term) => term.indent_fmt(f)?,
        None => return Ok(()),
    }
    for term in terms {
        f.write_str(", ")?;
        term.indent_fmt(f)?;
    }
    Ok(())
}

impl<'a> PrettyProvider<'a> {
    pub(crate) fn number_style(&self) -> ColorSpec {
        ColorSpec::new().set_fg(Some(Color::Rgb(206, 153, 100))).clone()
    }
    pub(crate) fn macro_style(&self) -> ColorSpec {
        ColorSpec::new().set_fg(Some(Color::Rgb(87, 182, 194))).clone()
    }
    pub(crate) fn keyword_style(&self) -> ColorSpec {
        ColorSpec::new().set_fg(Some(Color::Rgb(197, 119, 207))).clone()
    }
}
