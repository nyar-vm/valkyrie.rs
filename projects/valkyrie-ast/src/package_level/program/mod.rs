use super::*;

mod display;
mod parser;

/// The top level elements in script mode.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProgramRoot {
    /// All the statements in the program.
    pub statements: Vec<StatementKind>,
}
