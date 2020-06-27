use std::fmt::{Display, Formatter};

pub fn comma_terms<T: Display>(f: &mut Formatter<'_>, input: &[T]) -> std::fmt::Result {
    let mut terms = input.iter();
    match terms.next() {
        Some(term) => Display::fmt(term, f)?,
        None => return Ok(()),
    }
    for term in terms {
        write!(f, ", {}", term)?;
    }
    Ok(())
}
