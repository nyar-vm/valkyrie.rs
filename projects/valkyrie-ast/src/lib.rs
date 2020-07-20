#![no_std]
#![allow(incomplete_features)]
#![feature(adt_const_params)]

extern crate alloc;

mod control_flow;
mod expression_level;
mod package_level;
mod utils;

pub use crate::{
    control_flow::looping::{ConditionType, ForLoopNode, PatternType, WhileLoopNode},
    expression_level::{
        apply::{ApplyArgumentNode, ApplyCallNode, ApplyDotNode, ApplyTermNode, ArgumentKeyNode, ArgumentTermNode},
        ctor::{NewStructureCollectNode, NewStructureNode},
        generic::{GenericArgumentNode, GenericCall},
        lambda::{LambdaArgumentNode, LambdaNode},
        number::NumberLiteralNode,
        operators::{InfixNode, OperatorNode, PostfixNode, PrefixNode, ValkyrieOperator},
        string::StringLiteralNode,
        symbol::{IdentifierNode, MacroKind, MacroPathNode, NamePathNode},
        table::{TableKind, TableNode},
        view::{ViewNode, ViewRangeNode, ViewTermNode},
        ExpressionBody, ExpressionContext, ExpressionNode, ExpressionTermNode, ExpressionTypeNode,
    },
    package_level::{
        classes::ClassDeclarationNode,
        function::{FunctionCommonPart, FunctionDeclarationNode, FunctionType},
        import::{
            ImportAliasNode, ImportFlattenTerm, ImportGroupNode, ImportStatementNode, ImportStatementType, ImportTermNode,
        },
        namespace::{NamespaceDeclarationNode, NamespaceKind},
        StatementNode, StatementType,
    },
};
