#![no_std]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(try_blocks)]

extern crate alloc;

mod control_flow;
mod expression_level;
mod helper;
mod package_level;
mod string_like;

pub use crate::{
    control_flow::{
        control::{ControlNode, ControlType},
        if_else::{ConditionNode, ConditionType, ElsePart, IfStatementNode},
        looping::{ForLoop, PatternType, WhileLoop},
    },
    expression_level::{
        apply::{ApplyArgumentNode, ApplyArgumentTerm, ApplyCallNode, ApplyCallTerm, ApplyDotNode, ArgumentKeyNode},
        common::{ArgumentTermNode, CallNode, CallTermNode},
        ctor::NewConstructNode,
        generic::{GenericArgumentNode, GenericArgumentTerm, GenericCallNode, GenericCallTerm},
        lambda::{LambdaArgumentNode, LambdaCallNode, LambdaDotNode, LambdaNode},
        number::NumberLiteralNode,
        operators::{InfixNode, OperatorNode, PostfixNode, PrefixNode, ValkyrieOperator},
        pattern_match::MatchCaseNode,
        symbol::{IdentifierNode, MacroKind, MacroPathNode, NamePathNode, SlotNode},
        table::{TableKeyType, TableKind, TableNode, TableTermNode},
        view::{SubscriptNode, SubscriptSliceNode, SubscriptTermNode},
        ExpressionBody, ExpressionContext, ExpressionNode, PostfixCallPart,
    },
    helper::ValkyrieNode,
    package_level::{
        classes::{ClassDeclaration, ClassKind},
        documentation::DocumentationNode,
        function::{
            FunctionBody, FunctionDeclaration, FunctionDeclarationInline, FunctionReturnNode, FunctionType, ModifierPart,
        },
        import::{ImportAliasNode, ImportGroupNode, ImportResolvedItem, ImportStatementNode, ImportTermNode},
        let_bind::LetBindNode,
        namespace::{NamespaceDeclarationNode, NamespaceKind},
        try_catch::TryStatementNode,
        StatementContext, StatementNode, StatementType,
    },
    string_like::{
        string_formatter::StringFormatter,
        string_literal::StringLiteralNode,
        string_template::{StringTemplateNode, TemplateCloseNode, TemplateCommentNode, TemplateInlineNode, TemplateOpenNode},
    },
};
#[cfg(feature = "pretty-print")]
pub use pretty_print::{PrettyPrint, PrettyProvider, PrettyTree};
