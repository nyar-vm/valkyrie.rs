#![allow(dead_code, unused_imports, non_camel_case_types)]
#![allow(missing_docs, rustdoc::missing_crate_level_docs)]
#![allow(clippy::unnecessary_cast)]
#![doc = include_str!("readme.md")]

mod parse_ast;
mod parse_cst;

use core::str::FromStr;
use std::{borrow::Cow, ops::Range, sync::OnceLock};
use yggdrasil_rt::*;

type Input<'i> = Box<State<'i, ValkyrieRule>>;
type Output<'i> = Result<Box<State<'i, ValkyrieRule>>, Box<State<'i, ValkyrieRule>>>;

#[doc = include_str!("railway.min.svg")]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValkyrieParser {}

impl YggdrasilParser for ValkyrieParser {
    type Rule = ValkyrieRule;
    fn parse_cst(input: &str, rule: Self::Rule) -> OutputResult<ValkyrieRule> {
        self::parse_cst::parse_cst(input, rule)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ValkyrieRule {
    Program,
    Statement,
    EOS,
    EOS_FREE,
    DefineNamespace,
    OP_NAMESPACE,
    DefineImport,
    ImportTerm,
    ImportAs,
    ImportAll,
    ImportBlock,
    ImportMacro,
    ImportMacroItem,
    DefineTemplate,
    TemplateParameters,
    TemplateBlock,
    TemplateStatement,
    TemplateImplements,
    WhereBlock,
    WhereBound,
    DefineClass,
    ClassBlock,
    ClassBlockItem,
    ClassInherit,
    ClassInheritItem,
    ClassField,
    field_modifier,
    ParameterDefault,
    ClassMethod,
    method_modifier,
    ClassDomain,
    KW_CLASS,
    ObjectStatement,
    DefineUnion,
    KW_UNION,
    DefineEnumerate,
    EnumerateTerms,
    EnumerateField,
    KW_FLAGS,
    DefineTrait,
    KW_TRAIT,
    DefineFunction,
    KW_FUNCTION,
    Continuation,
    WhileStatement,
    KW_WHILE,
    ForStatement,
    MainStatement,
    ExpressionStatement,
    MatchStatement,
    MatchTerms,
    MatchType,
    KW_TYPE,
    KW_CASE,
    KW_MATCH,
    MainExpression,
    MainTerm,
    MainFactor,
    GroupFactor,
    Leading,
    MainInfix,
    MainPrefix,
    MainSuffix,
    InlineExpression,
    InlineTerm,
    InlineSuffix,
    SuffixOperator,
    TypeHint,
    TypeExpression,
    TypeTerm,
    TypeFactor,
    TypeInfix,
    TypePrefix,
    TypeSuffix,
    TryStatement,
    NewStatement,
    NewModifiers,
    NewBlock,
    NEW_MODIFIER_STOP,
    NewPair,
    NewPairKey,
    DotCall,
    DotCallItem,
    TupleCall,
    TupleLiteral,
    TupleLiteralStrict,
    TupleTerms,
    TuplePair,
    TupleKey,
    RangeCall,
    RangeLiteral,
    SubscriptAxis,
    SubscriptOnly,
    SubscriptRange,
    RangeOmit,
    GenericCall,
    GenericHide,
    GenericTerms,
    GenericPair,
    AttributeCall,
    ProceduralCall,
    TextLiteral,
    TextRaw,
    TEXT_CONTENT1,
    TEXT_CONTENT2,
    TEXT_CONTENT3,
    TEXT_CONTENT4,
    TEXT_CONTENT5,
    TEXT_CONTENT6,
    ModifierCall,
    AttributePath,
    ProceduralPath,
    NamepathFree,
    Namepath,
    Identifier,
    IdentifierBare,
    IdentifierRaw,
    IdentifierRawText,
    Special,
    Integer,
    PROPORTION,
    COLON,
    COMMA,
    DOT,
    OFFSET_L,
    OFFSET_R,
    PROPORTION2,
    OP_IMPORT_ALL,
    OP_AND_THEN,
    OP_BIND,
    KW_NAMESPACE,
    KW_IMPORT,
    KW_TEMPLATE,
    KW_WHERE,
    KW_IMPLEMENTS,
    KW_EXTENDS,
    KW_INHERITS,
    KW_IF,
    KW_ELSE,
    KW_FOR,
    KW_RETURN,
    KW_BREAK,
    KW_CONTINUE,
    KW_TRY,
    KW_NEW,
    KW_OBJECT,
    KW_NOT,
    KW_IN,
    KW_IS,
    KW_AS,
    WhiteSpace,
    SkipSpace,
    Comment,
    /// Label for unnamed text literal
    HiddenText,
}

impl YggdrasilRule for ValkyrieRule {
    fn is_ignore(&self) -> bool {
        matches!(self, Self::HiddenText | Self::SkipSpace | Self::Comment)
    }

    fn get_style(&self) -> &'static str {
        match self {
            Self::Program => "",
            Self::Statement => "",
            Self::EOS => "",
            Self::EOS_FREE => "",
            Self::DefineNamespace => "",
            Self::OP_NAMESPACE => "",
            Self::DefineImport => "",
            Self::ImportTerm => "",
            Self::ImportAs => "",
            Self::ImportAll => "",
            Self::ImportBlock => "",
            Self::ImportMacro => "",
            Self::ImportMacroItem => "",
            Self::DefineTemplate => "",
            Self::TemplateParameters => "",
            Self::TemplateBlock => "",
            Self::TemplateStatement => "",
            Self::TemplateImplements => "",
            Self::WhereBlock => "",
            Self::WhereBound => "",
            Self::DefineClass => "",
            Self::ClassBlock => "",
            Self::ClassBlockItem => "",
            Self::ClassInherit => "",
            Self::ClassInheritItem => "",
            Self::ClassField => "",
            Self::field_modifier => "",
            Self::ParameterDefault => "",
            Self::ClassMethod => "",
            Self::method_modifier => "",
            Self::ClassDomain => "",
            Self::KW_CLASS => "",
            Self::ObjectStatement => "",
            Self::DefineUnion => "",
            Self::KW_UNION => "",
            Self::DefineEnumerate => "",
            Self::EnumerateTerms => "",
            Self::EnumerateField => "",
            Self::KW_FLAGS => "",
            Self::DefineTrait => "",
            Self::KW_TRAIT => "",
            Self::DefineFunction => "",
            Self::KW_FUNCTION => "",
            Self::Continuation => "",
            Self::WhileStatement => "",
            Self::KW_WHILE => "",
            Self::ForStatement => "",
            Self::MainStatement => "",
            Self::ExpressionStatement => "",
            Self::MatchStatement => "",
            Self::MatchTerms => "",
            Self::MatchType => "",
            Self::KW_TYPE => "",
            Self::KW_CASE => "",
            Self::KW_MATCH => "",
            Self::MainExpression => "",
            Self::MainTerm => "",
            Self::MainFactor => "",
            Self::GroupFactor => "",
            Self::Leading => "",
            Self::MainInfix => "",
            Self::MainPrefix => "",
            Self::MainSuffix => "",
            Self::InlineExpression => "",
            Self::InlineTerm => "",
            Self::InlineSuffix => "",
            Self::SuffixOperator => "",
            Self::TypeHint => "",
            Self::TypeExpression => "",
            Self::TypeTerm => "",
            Self::TypeFactor => "",
            Self::TypeInfix => "",
            Self::TypePrefix => "",
            Self::TypeSuffix => "",
            Self::TryStatement => "",
            Self::NewStatement => "",
            Self::NewModifiers => "",
            Self::NewBlock => "",
            Self::NEW_MODIFIER_STOP => "",
            Self::NewPair => "",
            Self::NewPairKey => "",
            Self::DotCall => "",
            Self::DotCallItem => "",
            Self::TupleCall => "",
            Self::TupleLiteral => "",
            Self::TupleLiteralStrict => "",
            Self::TupleTerms => "",
            Self::TuplePair => "",
            Self::TupleKey => "",
            Self::RangeCall => "",
            Self::RangeLiteral => "",
            Self::SubscriptAxis => "",
            Self::SubscriptOnly => "",
            Self::SubscriptRange => "",
            Self::RangeOmit => "",
            Self::GenericCall => "",
            Self::GenericHide => "",
            Self::GenericTerms => "",
            Self::GenericPair => "",
            Self::AttributeCall => "",
            Self::ProceduralCall => "",
            Self::TextLiteral => "",
            Self::TextRaw => "",
            Self::TEXT_CONTENT1 => "",
            Self::TEXT_CONTENT2 => "",
            Self::TEXT_CONTENT3 => "",
            Self::TEXT_CONTENT4 => "",
            Self::TEXT_CONTENT5 => "",
            Self::TEXT_CONTENT6 => "",
            Self::ModifierCall => "",
            Self::AttributePath => "",
            Self::ProceduralPath => "",
            Self::NamepathFree => "",
            Self::Namepath => "",
            Self::Identifier => "",
            Self::IdentifierBare => "",
            Self::IdentifierRaw => "",
            Self::IdentifierRawText => "",
            Self::Special => "",
            Self::Integer => "",
            Self::PROPORTION => "",
            Self::COLON => "",
            Self::COMMA => "",
            Self::DOT => "",
            Self::OFFSET_L => "",
            Self::OFFSET_R => "",
            Self::PROPORTION2 => "",
            Self::OP_IMPORT_ALL => "",
            Self::OP_AND_THEN => "",
            Self::OP_BIND => "",
            Self::KW_NAMESPACE => "",
            Self::KW_IMPORT => "",
            Self::KW_TEMPLATE => "",
            Self::KW_WHERE => "",
            Self::KW_IMPLEMENTS => "",
            Self::KW_EXTENDS => "",
            Self::KW_INHERITS => "",
            Self::KW_IF => "",
            Self::KW_ELSE => "",
            Self::KW_FOR => "",
            Self::KW_RETURN => "",
            Self::KW_BREAK => "",
            Self::KW_CONTINUE => "",
            Self::KW_TRY => "",
            Self::KW_NEW => "",
            Self::KW_OBJECT => "",
            Self::KW_NOT => "",
            Self::KW_IN => "",
            Self::KW_IS => "",
            Self::KW_AS => "",
            Self::WhiteSpace => "",
            Self::SkipSpace => "",
            Self::Comment => "",
            _ => "",
        }
    }
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProgramNode {
    pub statement: Vec<StatementNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StatementNode {
    DefineClass(DefineClassNode),
    DefineEnumerate(DefineEnumerateNode),
    DefineFunction(DefineFunctionNode),
    DefineImport(DefineImportNode),
    DefineNamespace(DefineNamespaceNode),
    DefineTrait(DefineTraitNode),
    DefineUnion(DefineUnionNode),
    MainStatement(MainStatementNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EosNode {
    Omit,
    Show,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EosFreeNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DefineNamespaceNode {
    pub namepath_free: NamepathFreeNode,
    pub op_namespace: Option<OpNamespaceNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OpNamespaceNode {
    Hide,
    Main,
    Test,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DefineImportNode {
    pub import_term: Vec<ImportTermNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImportTermNode {
    ImportAll(ImportAllNode),
    ImportAs(ImportAsNode),
    ImportBlock(ImportBlockNode),
    ImportMacro(ImportMacroNode),
    NamepathFree(NamepathFreeNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportAsNode {
    pub namepath_free: NamepathFreeNode,
    pub alias: IdentifierNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportAllNode {
    pub namepath_free: NamepathFreeNode,
    pub op_import_all: OpImportAllNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportBlockNode {
    pub import_term: Vec<ImportTermNode>,
    pub namepath_free: NamepathFreeNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportMacroNode {
    pub import_macro_item: ImportMacroItemNode,
    pub namepath_free: NamepathFreeNode,
    pub alias: ImportMacroItemNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImportMacroItemNode {
    Capture(IdentifierNode),
    Instant(IdentifierNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DefineTemplateNode {
    pub attribute_call: Vec<AttributeCallNode>,
    pub kw_template: KwTemplateNode,
    pub modifier_call: Vec<ModifierCallNode>,
    pub template_block: TemplateBlockNode,
    pub template_parameters: Option<TemplateParametersNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateParametersNode {
    pub identifier: Vec<IdentifierNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateBlockNode {
    pub eos_free: Vec<EosFreeNode>,
    pub template_implements: Vec<TemplateImplementsNode>,
    pub template_statement: Vec<TemplateStatementNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateStatementNode {
    pub where_block: WhereBlockNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateImplementsNode {
    pub kw_implements: KwImplementsNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhereBlockNode {
    pub kw_where: KwWhereNode,
    pub where_bound: Vec<WhereBoundNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhereBoundNode {
    pub eos_free: EosFreeNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DefineClassNode {
    pub attribute_call: Vec<AttributeCallNode>,
    pub class_block: ClassBlockNode,
    pub class_inherit: Option<ClassInheritNode>,
    pub define_template: Option<DefineTemplateNode>,
    pub identifier: IdentifierNode,
    pub kw_class: KwClassNode,
    pub modifier_call: Vec<ModifierCallNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassBlockNode {
    pub class_block_item: Vec<ClassBlockItemNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClassBlockItemNode {
    ClassDomain(ClassDomainNode),
    ClassField(ClassFieldNode),
    ClassMethod(ClassMethodNode),
    EosFree(EosFreeNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassInheritNode {
    pub class_inherit_item: Vec<ClassInheritItemNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassInheritItemNode {
    pub namepath: NamepathNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassFieldNode {
    pub attribute_call: Vec<AttributeCallNode>,
    pub identifier: IdentifierNode,
    pub parameter_default: Option<ParameterDefaultNode>,
    pub type_hint: Option<TypeHintNode>,
    pub field_modifier: Vec<FieldModifierNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FieldModifierNode {
    pub modifier_call: ModifierCallNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParameterDefaultNode {
    pub main_expression: MainExpressionNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassMethodNode {
    pub attribute_call: Vec<AttributeCallNode>,
    pub namepath: NamepathNode,
    pub method_modifier: Vec<MethodModifierNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MethodModifierNode {
    pub modifier_call: ModifierCallNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassDomainNode {
    pub attribute_call: Vec<AttributeCallNode>,
    pub class_block: ClassBlockNode,
    pub identifier: IdentifierNode,
    pub field_modifier: Vec<FieldModifierNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KwClassNode {
    Class,
    Structure,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectStatementNode {
    pub class_block: ClassBlockNode,
    pub class_inherit: Option<ClassInheritNode>,
    // Missing rule KW_Object
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DefineUnionNode {
    pub attribute_call: Vec<AttributeCallNode>,
    pub identifier: IdentifierNode,
    pub kw_union: KwUnionNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwUnionNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DefineEnumerateNode {
    pub attribute_call: Vec<AttributeCallNode>,
    pub enumerate_terms: Vec<EnumerateTermsNode>,
    pub identifier: IdentifierNode,
    pub kw_flags: KwFlagsNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EnumerateTermsNode {
    EnumerateField(EnumerateFieldNode),
    EosFree(EosFreeNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumerateFieldNode {
    pub identifier: IdentifierNode,
    pub main_expression: MainExpressionNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KwFlagsNode {
    Enum,
    Flags,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DefineTraitNode {
    pub kw_trait: KwTraitNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwTraitNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DefineFunctionNode {
    pub kw_function: KwFunctionNode,
    pub namepath: NamepathNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KwFunctionNode {
    Macro,
    Micro,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContinuationNode {
    pub main_statement: Vec<MainStatementNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhileStatementNode {
    pub continuation: ContinuationNode,
    pub inline_expression: Option<InlineExpressionNode>,
    pub kw_while: KwWhileNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KwWhileNode {
    Until,
    While,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForStatementNode {
    pub continuation: ContinuationNode,
    pub identifier: IdentifierNode,
    pub inline_expression: Option<InlineExpressionNode>,
    pub kw_for: KwForNode,
    pub kw_in: KwInNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MainStatementNode {
    ExpressionStatement(ExpressionStatementNode),
    ForStatement(ForStatementNode),
    WhileStatement(WhileStatementNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionStatementNode {
    pub eos: Option<EosNode>,
    pub main_expression: MainExpressionNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatchStatementNode {
    pub identifier: Option<IdentifierNode>,
    pub inline_expression: Option<InlineExpressionNode>,
    pub kw_match: KwMatchNode,
    pub match_terms: Vec<MatchTermsNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MatchTermsNode {
    MatchType(MatchTypeNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatchTypeNode {
    pub colon: ColonNode,
    pub identifier: IdentifierNode,
    pub kw_type: KwTypeNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwTypeNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwCaseNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KwMatchNode {
    Until,
    While,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MainExpressionNode {
    pub main_infix: Vec<MainInfixNode>,
    pub main_term: Vec<MainTermNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MainTermNode {
    pub main_factor: MainFactorNode,
    pub main_prefix: Vec<MainPrefixNode>,
    pub main_suffix: Vec<MainSuffixNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MainFactorNode {
    GroupFactor(GroupFactorNode),
    Leading(LeadingNode),
    MatchStatement(MatchStatementNode),
    NewStatement(NewStatementNode),
    ObjectStatement(ObjectStatementNode),
    TryStatement(TryStatementNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupFactorNode {
    pub main_expression: MainExpressionNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LeadingNode {
    Integer(IntegerNode),
    Namepath(NamepathNode),
    ProceduralCall(ProceduralCallNode),
    RangeLiteral(RangeLiteralNode),
    Special(SpecialNode),
    TextLiteral(TextLiteralNode),
    TupleLiteralStrict(TupleLiteralStrictNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MainInfixNode {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MainPrefixNode {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MainSuffixNode {
    InlineSuffix(InlineSuffixNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InlineExpressionNode {
    pub inline_term: Vec<InlineTermNode>,
    pub main_infix: Vec<MainInfixNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InlineTermNode {
    pub inline_suffix: Vec<InlineSuffixNode>,
    pub main_factor: MainFactorNode,
    pub main_prefix: Vec<MainPrefixNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InlineSuffixNode {
    DotCall(DotCallNode),
    GenericCall(GenericCallNode),
    InlineSuffix0(SuffixOperatorNode),
    RangeCall(RangeCallNode),
    TupleCall(TupleCallNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SuffixOperatorNode {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeHintNode {
    pub colon: ColonNode,
    pub type_expression: TypeExpressionNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeExpressionNode {
    pub type_infix: Vec<TypeInfixNode>,
    pub type_term: Vec<TypeTermNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeTermNode {
    pub main_factor: MainFactorNode,
    pub type_prefix: Vec<TypePrefixNode>,
    pub type_suffix: Vec<TypeSuffixNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypeFactorNode {
    Leading(LeadingNode),
    TypeFactor0(TypeExpressionNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeInfixNode {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypePrefixNode {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypeSuffixNode {
    GenericHide(GenericHideNode),
    Option,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TryStatementNode {
    pub continuation: ContinuationNode,
    pub kw_try: KwTryNode,
    pub type_expression: Option<TypeExpressionNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewStatementNode {
    pub generic_hide: Option<GenericHideNode>,
    pub kw_new: KwNewNode,
    pub namepath: NamepathNode,
    pub new_block: Option<NewBlockNode>,
    pub new_modifiers: Vec<NewModifiersNode>,
    pub tuple_literal: Option<TupleLiteralNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewModifiersNode {
    pub identifier: IdentifierNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewBlockNode {
    pub eos_free: Vec<EosFreeNode>,
    pub new_pair: Vec<NewPairNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewModifierStopNode {
    pub identifier: IdentifierNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewPairNode {
    // Missing rule Colon
    pub main_expression: MainExpressionNode,
    pub new_pair_key: Option<NewPairKeyNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NewPairKeyNode {
    Identifier(IdentifierNode),
    RangeLiteral(RangeLiteralNode),
    TextRaw(TextRawNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DotCallNode {
    pub dot_call_item: DotCallItemNode,
    pub op_and_then: Option<OpAndThenNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DotCallItemNode {
    Integer(IntegerNode),
    Namepath(NamepathNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TupleCallNode {
    pub continuation: Option<ContinuationNode>,
    pub op_and_then: Option<OpAndThenNode>,
    pub tuple_literal: Option<TupleLiteralNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TupleLiteralNode {
    pub tuple_terms: TupleTermsNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TupleLiteralStrictNode {
    pub tuple_pair: Vec<TuplePairNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TupleTermsNode {
    pub tuple_pair: Vec<TuplePairNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TuplePairNode {
    // Missing rule Colon
    pub main_expression: MainExpressionNode,
    pub tuple_key: Option<TupleKeyNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TupleKeyNode {
    Identifier(IdentifierNode),
    TextRaw(TextRawNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RangeCallNode {
    pub op_and_then: Option<OpAndThenNode>,
    pub range_literal: RangeLiteralNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RangeLiteralNode {
    pub subscript_axis: Vec<SubscriptAxisNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SubscriptAxisNode {
    SubscriptOnly(SubscriptOnlyNode),
    SubscriptRange(SubscriptRangeNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubscriptOnlyNode {
    pub index: MainExpressionNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubscriptRangeNode {
    pub head: Option<MainExpressionNode>,
    pub step: Option<MainExpressionNode>,
    pub tail: Option<MainExpressionNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RangeOmitNode {
    pub colon: Vec<ColonNode>,
    pub proportion: Option<ProportionNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericCallNode {
    pub generic_terms: GenericTermsNode,
    pub namepath: Option<NamepathNode>,
    pub op_and_then: Option<OpAndThenNode>,
    pub proportion: Vec<ProportionNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericHideNode {
    pub generic_terms: GenericTermsNode,
    pub proportion: Option<ProportionNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericTermsNode {
    pub generic_pair: Vec<GenericPairNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericPairNode {
    // Missing rule Colon
    pub identifier: Option<IdentifierNode>,
    pub type_expression: TypeExpressionNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeCallNode {
    pub attribute_path: AttributePathNode,
    pub tuple_literal: Option<TupleLiteralNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProceduralCallNode {
    pub procedural_path: ProceduralPathNode,
    pub tuple_literal: Option<TupleLiteralNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextLiteralNode {
    pub identifier: Option<IdentifierNode>,
    pub text_raw: TextRawNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextRawNode {
    pub text_content_1: Option<TextContent1Node>,
    pub text_content_2: Option<TextContent2Node>,
    pub text_content_3: Option<TextContent3Node>,
    pub text_content_4: Option<TextContent4Node>,
    pub text_content_5: Option<TextContent5Node>,
    pub text_content_6: Option<TextContent6Node>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextContent1Node {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextContent2Node {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextContent3Node {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextContent4Node {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextContent5Node {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextContent6Node {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifierCallNode {
    pub identifier: IdentifierNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributePathNode {
    pub namepath: NamepathNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProceduralPathNode {
    pub namepath: NamepathNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamepathFreeNode {
    pub identifier: Vec<IdentifierNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamepathNode {
    pub identifier: Vec<IdentifierNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IdentifierNode {
    IdentifierBare(IdentifierBareNode),
    IdentifierRaw(IdentifierRawNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentifierBareNode {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentifierRawNode {
    pub identifier_raw_text: IdentifierRawTextNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentifierRawTextNode {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpecialNode {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntegerNode {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProportionNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColonNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommaNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DotNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OffsetLNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OffsetRNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Proportion2Node {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpImportAllNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpAndThenNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpBindNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwNamespaceNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwImportNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwTemplateNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwWhereNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwImplementsNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwExtendsNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwInheritsNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwIfNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwElseNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwForNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwReturnNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwBreakNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwContinueNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwTryNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwNewNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwObjectNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwNotNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwInNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwIsNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwAsNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhiteSpaceNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkipSpaceNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommentNode {
    pub span: Range<u32>,
}
