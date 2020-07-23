use core::fmt::{Display, Formatter, Write};
use indentation::{IndentDisplay, IndentFormatter};

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
