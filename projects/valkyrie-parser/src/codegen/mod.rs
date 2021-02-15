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
    DefineClass,
    ClassBlock,
    ClassBlockItem,
    ClassInherit,
    ClassInheritItem,
    ClassField,
    ClassMethod,
    ClassDomain,
    DefineTemplate,
    TemplateParameters,
    TemplateBlock,
    TemplateStatement,
    /// Label for text literal
    IgnoreText,
    /// Label for regex literal
    IgnoreRegex,
}

impl YggdrasilRule for ValkyrieRule {
    fn is_ignore(&self) -> bool {
        matches!(self, Self::IgnoreText | Self::IgnoreRegex)
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
            Self::DefineClass => "",
            Self::ClassBlock => "",
            Self::ClassBlockItem => "",
            Self::ClassInherit => "",
            Self::ClassInheritItem => "",
            Self::ClassField => "",
            Self::ClassMethod => "",
            Self::ClassDomain => "",
            Self::DefineTemplate => "",
            Self::TemplateParameters => "",
            Self::TemplateBlock => "",
            Self::TemplateStatement => "",
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
    DefineImport(DefineImportNode),
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
    // Missing rule NamepathFree
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
    NamepathFree(),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportAsNode {
    // Missing rule NamepathFree
    // Missing rule Identifier
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportAllNode {
    // Missing rule NamepathFree
    pub op_import_all: OpImportAllNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportBlockNode {
    pub import_term: Vec<ImportTermNode>,
    // Missing rule NamepathFree
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportMacroNode {
    pub import_macro_item: ImportMacroItemNode,
    // Missing rule NamepathFree
    pub alias: ImportMacroItemNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImportMacroItemNode {
    Capture(),
    Instant(),
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
pub struct DefineClassNode {
    pub class_block: ClassBlockNode,
    pub class_inherit: Option<ClassInheritNode>,
    // Missing rule DefineGeneric
    pub define_template: Option<DefineTemplateNode>,
    // Missing rule Identifier
    // Missing rule KW_CLASS
    // Missing rule TypeHint
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
    // Missing rule Namepath
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassFieldNode {
    // Missing rule Identifier
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassMethodNode {
    // Missing rule Identifier
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassDomainNode {
    pub class_block: ClassBlockNode,
    // Missing rule Identifier
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DefineTemplateNode {
    // Missing rule KW_TEMPLATE
    pub template_block: TemplateBlockNode,
    pub template_parameters: Option<TemplateParametersNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateParametersNode {
    // Missing rule COMMA
    // Missing rule Identifier
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateBlockNode {
    pub eos_free: Vec<EosFreeNode>,
    // Missing rule TemplateImplements
    pub template_statement: Vec<TemplateStatementNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemplateStatementNode {
    // Missing rule WhereBlock
    pub span: Range<u32>,
}
