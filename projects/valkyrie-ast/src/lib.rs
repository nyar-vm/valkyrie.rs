#![no_std]
#![deny(missing_debug_implementations, missing_copy_implementations)]
// #![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/91894079")]

extern crate alloc;

mod control_flow;
mod expression_level;
pub mod helper;
mod package_level;
mod patterns;
mod string_like;

pub use crate::{
    control_flow::{
        control::{ControlNode, ControlType, RaiseNode},
        jmp_guard::{GuardLetStatement, GuardStatement, GuardStatementBody},
        jmp_if::{ElseStatement, IfConditionNode, IfLetStatement, IfStatement, ThenStatement},
        jmp_switch::SwitchStatement,
        loop_for::{ForBarePattern, ForLoop},
        loop_while::{OtherwiseStatement, WhileConditionNode, WhileLoop, WhileLoopKind},
    },
    expression_level::{
        annotations::{AnnotationKind, AnnotationList, AnnotationNode, AnnotationPathNode, AnnotationTerm, ModifiersNode},
        apply::{ApplyArgument, ApplyArgumentTerm, ApplyCallNode, ApplyCallTerm, ApplyDotNode, ArgumentKeyNode},
        common::{ArgumentTermNode, CallNode, CallTermNode, MonadicCall, MonadicDotCall},
        ctor::{CollectsNode, NewConstructNode},
        generic::{GenericArgument, GenericArgumentTerm, GenericCallNode, GenericCallTerm},
        lambda::{LambdaArgumentNode, LambdaCallNode, LambdaDotNode, LambdaNode},
        matches::{MatchDotStatement, MatchKind},
        number::NumberLiteralNode,
        operators::{InfixNode, OperatorNode, PostfixNode, PrefixNode, ValkyrieOperator},
        symbol::{IdentifierNode, LambdaSlotNode, NamePathNode},
        table::{TableKeyType, TableKind, TableNode, TableTermNode},
        view::{SubscriptNode, SubscriptSliceNode, SubscriptTermNode},
        ExpressionContext, ExpressionNode, ExpressionType, PostfixCallPart, TypingExpression,
    },
    package_level::{
        classes::{ClassDeclaration, ClassFieldDeclaration, ClassIterator, ClassKind, ClassMethodDeclaration, ClassTerm},
        documentation::DocumentationNode,
        enumerates::{EnumerateDeclaration, EnumerateFieldDeclaration},
        flags::{FlagsDeclaration, FlagsIterator, FlagsTerm},
        function::{
            FunctionDeclaration, FunctionDeclarationInline, FunctionEffectNode, FunctionReturnNode, FunctionType,
            StatementBlock,
        },
        guarantee::{EffectTypeNode, GuaranteeNode},
        import::{ImportAliasNode, ImportGroupNode, ImportResolvedItem, ImportState, ImportStatement, ImportTermNode},
        labeled::{GotoStatement, LabelStatement},
        let_bind::LetBindNode,
        namespace::{NamespaceDeclaration, NamespaceKind},
        program::ProgramRoot,
        statements::{StatementContext, StatementNode, StatementType},
        tagged::{TaggedDeclaration, VariantDeclaration},
        try_catch::TryStatement,
        unions::{UnionDeclaration, UnionFieldDeclaration},
    },
    patterns::{
        ArrayPatternNode, ClassPatternNode, ImplicitCaseNode, PatternBlock, PatternBranch, PatternCaseNode, PatternCondition,
        PatternElseNode, PatternExpressionType, PatternGuard, PatternStatements, PatternTypeNode, PatternWhenNode,
        TuplePatternNode, UnionPatternNode,
    },
    string_like::{
        string_formatter::StringFormatter,
        string_literal::{StringLiteralNode, StringTextNode},
        string_template::{
            StringTemplateNode, TemplateCloseNode, TemplateCommentNode, TemplateInlineNode, TemplateLineType, TemplateOpenNode,
        },
    },
};
