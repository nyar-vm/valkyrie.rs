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
        guard_statement::{GuardPattern, GuardStatement},
        jmp_if::{ElseStatement, IfConditionNode, IfStatement},
        jmp_switch::SwitchStatement,
        loop_for::ForLoop,
        loop_while::{PatternExpression, WhileConditionNode, WhileLoop},
        pattern::{
            PatternBranch, PatternCaseNode, PatternCondition, PatternElseNode, PatternGuard, PatternStatements,
            PatternTypeNode, PatternWhenNode,
        },
    },
    expression_level::{
        annotations::{AnnotationKind, AnnotationList, AnnotationNode, AnnotationPathNode, AnnotationTerm, ModifiersNode},
        apply::{ApplyArgumentNode, ApplyArgumentTerm, ApplyCallNode, ApplyCallTerm, ApplyDotNode, ArgumentKeyNode},
        common::{ArgumentTermNode, CallNode, CallTermNode},
        ctor::{CollectsNode, NewConstructNode},
        generic::{GenericArgumentNode, GenericArgumentTerm, GenericCallNode, GenericCallTerm},
        lambda::{LambdaArgumentNode, LambdaCallNode, LambdaDotNode, LambdaNode},
        number::NumberLiteralNode,
        operators::{InfixNode, OperatorNode, PostfixNode, PrefixNode, ValkyrieOperator},
        pattern_match::{MatchKind, MatchStatement},
        symbol::{IdentifierNode, LambdaSlotNode, NamePathNode},
        table::{TableKeyType, TableKind, TableNode, TableTermNode},
        view::{SubscriptNode, SubscriptSliceNode, SubscriptTermNode},
        ExpressionBody, ExpressionContext, ExpressionNode, PostfixCallPart, TypingExpression,
    },
    helper::ValkyrieNode,
    package_level::{
        classes::{ClassDeclaration, ClassFieldDeclaration, ClassIterator, ClassKind, ClassMethodDeclaration, ClassTerm},
        documentation::DocumentationNode,
        enumerates::{EnumerateDeclaration, EnumerateFieldDeclaration},
        flags::{FlagsDeclaration, FlagsIterator, FlagsTerm},
        function::{FunctionDeclaration, FunctionDeclarationInline, FunctionReturnNode, FunctionType, StatementBlock},
        guarantee::{EffectTypeNode, GuaranteeNode},
        import::{ImportAliasNode, ImportGroupNode, ImportResolvedItem, ImportStatement, ImportTermNode},
        let_bind::LetBindNode,
        namespace::{NamespaceDeclaration, NamespaceKind},
        program::ProgramRoot,
        statements::{StatementBody, StatementContext, StatementNode},
        tagged::{TaggedDeclaration, VariantDeclaration},
        try_catch::TryStatementNode,
        unions::{UnionDeclaration, UnionFieldDeclaration},
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
