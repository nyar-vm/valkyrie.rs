#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(try_blocks)]

mod control_flow;
mod expression_level;
mod helper;
mod package_level;
mod utils;

pub use crate::{
    control_flow::{
        control::{ControlNode, ControlType},
        if_else::{ConditionNode, ConditionType, ElsePart, IfStatementNode},
        looping::{ForLoopNode, PatternType, WhileLoopNode},
    },
    expression_level::{
        apply::{ApplyArgumentNode, ApplyCallNode, ApplyDotNode, ArgumentKeyNode, ArgumentTermNode, MaybePair},
        ctor::{NewStructureCollectNode, NewStructureNode},
        generic::{GenericArgumentNode, GenericCall},
        lambda::{LambdaArgumentNode, LambdaCallNode, LambdaDotNode, LambdaNode},
        number::NumberLiteralNode,
        operators::{InfixNode, OperatorNode, PostfixNode, PrefixNode, ValkyrieOperator},
        string::StringLiteralNode,
        symbol::{IdentifierNode, MacroKind, MacroPathNode, NamePathNode},
        table::{TableKeyKind, TableKind, TableNode, TableTermNode},
        view::{ViewNode, ViewRangeNode, ViewTermNode},
        ExpressionBody, ExpressionContext, ExpressionNode, ExpressionType,
    },
    helper::{PrettyPrint, PrettyProvider, PrettyTree},
    package_level::{
        classes::{ClassDeclaration, ClassKind},
        function::{
            FunctionBodyPart, FunctionCommonPart, FunctionDeclaration, FunctionDeclarationInline, FunctionType, ModifierPart,
        },
        import::{
            ImportAliasNode, ImportFlattenTerm, ImportGroupNode, ImportStatementNode, ImportStatementType, ImportTermNode,
        },
        namespace::{NamespaceDeclarationNode, NamespaceKind},
        StatementContext, StatementNode, StatementType,
    },
};
