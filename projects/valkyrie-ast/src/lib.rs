#![no_std]
#![allow(unused_imports)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
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
        control::{ControlNode, ControlType, RaiseNode, TailCallNode},
        jmp_guard::{GuardPattern, GuardStatement},
        jmp_if::{BreakStatement, ElseStatement, IfBranchNode, IfStatement, JumpStatement},
        jmp_switch::SwitchStatement,
        loop_for::{ForBarePattern, ForLoop},
        loop_while::{OtherwiseStatement, WhileConditionNode, WhileLoop, WhileLoopKind},
    },
    expression_level::{
        annotations::{
            AnnotationKind, AnnotationList, AnnotationNode, AnnotationPathNode, AnnotationTerm, ModifiedNode, ModifiersNode,
        },
        argument::{ApplyCallNode, ArgumentKey, ArgumentTerm, ArgumentsList},
        common::{ArgumentTermNode, CallNode},
        ctor::{CollectsNode, ConstructNewNode},
        dot_call::{DotCallNode, DotCallTerm},
        generic_call::GenericCallNode,
        lambda::{ClosureCallNode, ClosureCaller, FunctionBlock, LambdaNode},
        matches::{MatchDotStatement, MatchKind},
        number::NumberLiteralNode,
        operators::{BinaryNode, LogicMatrix, OperatorNode, UnaryNode, ValkyrieOperator},
        parameter::{ParameterKind, ParameterTerm, ParametersList},
        range::{RangeKind, RangeNode, RangeTermNode, SubscriptCallNode},
        symbol::{BooleanNode, IdentifierNode, LambdaSlotNode, NamePathNode, NullNode, OutputNode},
        tuple::{TupleKind, TupleNode, TupleTermNode},
        ExpressionContext, ExpressionNode, ExpressionType, PostfixCallPart, TypingExpression,
    },
    package_level::{
        classes::{ClassDeclaration, ClassFieldDeclaration, ClassKind, ClassMethodDeclaration, ConstructObjectNode},
        documentation::DocumentationNode,
        flags::{FlagDeclaration, FlagFieldDeclaration, FlagsKind},
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
        statements::{StatementContext, StatementNode},
        tagged::{TaggedDeclaration, VariantDeclaration},
        traits::{ExtendsStatement, TraitDeclaration},
        try_catch::TryStatement,
        unions::{UnionDeclaration, UnionFieldDeclaration},
    },
    patterns::{
        ArrayPatternNode, ClassPatternNode, IdentifierPattern, ImplicitCaseNode, LetPattern, PatternBlock, PatternBranch,
        PatternCaseNode, PatternCondition, PatternElseNode, PatternGuard, PatternStatements, PatternTypeNode, PatternWhenNode,
        TuplePatternNode, UnionPatternNode,
    },
    string_like::{
        string_formatter::{ExpressionFormatted, StringFormatter},
        string_literal::{StringLiteralNode, StringTextNode},
        string_template::{
            StringTemplateNode, TemplateCloseNode, TemplateCommentNode, TemplateInlineNode, TemplateLineType, TemplateOpenNode,
        },
    },
};
pub use num_bigint::BigUint;
