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
        for_loop::ForLoop,
        guard_statement::{GuardPattern, GuardStatement},
        if_else::{ConditionNode, ConditionType, ElsePart, IfStatement},
        looping::{PatternType, WhileLoop},
        pattern::PatternCondition,
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
        symbol::{IdentifierNode, LambdaSlotNode, MacroKind, MacroPathNode, NamePathNode},
        table::{TableKeyType, TableKind, TableNode, TableTermNode},
        view::{SubscriptNode, SubscriptSliceNode, SubscriptTermNode},
        ExpressionBody, ExpressionContext, ExpressionNode, PostfixCallPart, TypingExpression,
    },
    helper::ValkyrieNode,
    package_level::{
        classes::{ClassDeclaration, ClassFieldDeclaration, ClassKind},
        documentation::DocumentationNode,
        enumerates::{EnumerateDeclaration, EnumerateFieldDeclaration},
        flags::{FlagsDeclaration, FlagsFieldDeclaration},
        function::{FunctionDeclaration, FunctionDeclarationInline, FunctionReturnNode, FunctionType, StatementBlock},
        guarantee::{EffectTypeNode, GuaranteeNode},
        import::{ImportAliasNode, ImportGroupNode, ImportResolvedItem, ImportStatementNode, ImportTermNode},
        let_bind::LetBindNode,
        namespace::{NamespaceDeclaration, NamespaceKind},
        tagged::{ModifiersNode, TaggedDeclaration, VariantDeclaration},
        try_catch::TryStatementNode,
        unions::{UnionDeclaration, UnionFieldDeclaration},
        StatementBody, StatementContext, StatementNode,
    },
    string_like::{
        string_formatter::StringFormatter,
        string_literal::StringLiteralNode,
        string_template::{
            StringTemplateNode, TemplateCloseNode, TemplateCommentNode, TemplateInlineNode, TemplateLineType, TemplateOpenNode,
        },
    },
};
#[cfg(feature = "pretty-print")]
pub use pretty_print::{PrettyPrint, PrettyProvider, PrettyTree};
