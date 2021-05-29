mod jmp_match;
mod jmp_switch;
mod loop_for;
mod loop_while;
use crate::{helpers::ProgramState, KwMatchNode};
use nyar_error::{Result, Success, Validation};
use valkyrie_ast::{
    IdentifierNode, MatchKind, MatchStatement, PatternBranch, PatternCaseNode, PatternCondition, PatternStatements,
    SwitchStatement, WhileConditionNode, WhileLoop, WhileLoopKind,
};
