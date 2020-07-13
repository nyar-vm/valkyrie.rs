use super::*;

pub(crate) fn format_else_body(f: &mut IndentFormatter, body: &[StatementNode]) -> core::fmt::Result {
    if body.is_empty() {
        return Ok(());
    }
    f.write_newline()?;
    f.write_str("else {")?;
    f.indent();
    for node in body {
        f.write_newline()?;
        node.indent_fmt(f)?;
    }
    f.dedent();
    f.write_newline()?;
    f.write_char('}')
}
