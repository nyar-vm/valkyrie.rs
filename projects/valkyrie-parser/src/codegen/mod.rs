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
    KW_NAMESPACE,
    OP_NAMESPACE,
    DefineImport,
    ImportTerm,
    ImportAs,
    ImportAll,
    ImportBlock,
    ImportMacro,
    ImportMacroItem,
    KW_IMPORT,
    OP_IMPORT_ALL,
    KW_CLASS,
    KW_UNION,
    KW_TRAIT,
    NamepathFree,
    Namepath,
    Identifier,
    IdentifierBare,
    IdentifierRaw,
    IdentifierRawText,
    Boolean,
    Integer,
    RangeExact,
    Range,
    ModifierCall,
    KW_AS,
    WhiteSpace,
    Comment,
    /// Label for text literal
    IgnoreText,
    /// Label for regex literal
    IgnoreRegex,
}

impl YggdrasilRule for ValkyrieRule {
    fn is_ignore(&self) -> bool {
        matches!(self, Self::IgnoreText | Self::IgnoreRegex | Self::WhiteSpace | Self::Comment)
    }

    fn get_style(&self) -> &'static str {
        match self {
            Self::Program => "",
            Self::Statement => "",
            Self::EOS => "",
            Self::EOS_FREE => "",
            Self::DefineNamespace => "",
            Self::KW_NAMESPACE => "",
            Self::OP_NAMESPACE => "",
            Self::DefineImport => "",
            Self::ImportTerm => "",
            Self::ImportAs => "",
            Self::ImportAll => "",
            Self::ImportBlock => "",
            Self::ImportMacro => "",
            Self::ImportMacroItem => "",
            Self::KW_IMPORT => "",
            Self::OP_IMPORT_ALL => "",
            Self::KW_CLASS => "",
            Self::KW_UNION => "",
            Self::KW_TRAIT => "",
            Self::NamepathFree => "",
            Self::Namepath => "",
            Self::Identifier => "",
            Self::IdentifierBare => "",
            Self::IdentifierRaw => "",
            Self::IdentifierRawText => "",
            Self::Boolean => "",
            Self::Integer => "",
            Self::RangeExact => "",
            Self::Range => "",
            Self::ModifierCall => "",
            Self::KW_AS => "",
            Self::WhiteSpace => "",
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
    DefineNamespace(DefineNamespaceNode),
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
pub struct KwNamespaceNode {
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
pub struct KwImportNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpImportAllNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwClassNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwUnionNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwTraitNode {
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
pub enum BooleanNode {
    False,
    True,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntegerNode {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RangeExactNode {
    pub integer: IntegerNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RangeNode {
    pub max: Option<IntegerNode>,
    pub min: Option<IntegerNode>,
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
pub struct CommentNode {
    pub span: Range<u32>,
}
