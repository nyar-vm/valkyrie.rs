mod jmp_match;
mod jmp_switch;
mod loop_for;
mod loop_while;
use crate::{helpers::ProgramState, utils::build_if_guard};
use nyar_error::Result;
use valkyrie_ast::{
    IdentifierNode, IdentifierPattern, MatchKind, MatchStatement, PatternBranch, PatternCaseNode, PatternCondition,
    PatternNode, PatternTypeNode, PatternWhenNode, PatternsList, StatementBlock, SwitchStatement, WhileConditionNode,
    WhileLoop, WhileLoopKind,
};
mod jmp_if;
