mod jmp_match;
mod jmp_switch;
mod loop_for;
mod loop_while;
use crate::{helpers::ProgramState, KwMatchNode, MatchTermsNode};
use nyar_error::{Success, Validation};
use valkyrie_ast::{
    IdentifierNode, MatchKind, MatchStatement, PatternBlock, SwitchStatement, WhileConditionNode, WhileLoop, WhileLoopKind,
};
