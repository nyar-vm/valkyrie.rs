use super::*;

// if a {1}
// if a {1} else {2}
// if a {1} else if b {2}
// if a {1} else if b {2} else {3}
// if a {1} else if b {2} else if c {3}
// if a {1} else if b {2} else if c {3} else {4}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfStatementNode {
    pub branches: Vec<ConditionNode>,
    pub else_branch: Vec<StatementNode>,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConditionNode {
    pub condition: ConditionType,
    pub body: Vec<StatementNode>,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConditionType {
    AlwaysTrue,
    Case,
    Expression(Box<ExpressionTermNode>),
}

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
