use super::*;
#[automatically_derived]
impl YggdrasilNode for ProgramNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            statement: pair.take_tagged_items::<StatementNode>(Cow::Borrowed("statement")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ProgramNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::Program)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for StatementNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::DefineImport(s) => s.get_range(),
            Self::DefineNamespace(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<DefineImportNode>(Cow::Borrowed("define_import")) {
            return Ok(Self::DefineImport(s));
        }
        if let Ok(s) = pair.take_tagged_one::<DefineNamespaceNode>(Cow::Borrowed("define_namespace")) {
            return Ok(Self::DefineNamespace(s));
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::Statement, _span))
    }
}
#[automatically_derived]
impl FromStr for StatementNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::Statement)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for EosNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Omit => None,
            Self::Show => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("omit") {
            return Ok(Self::Omit);
        }
        if let Some(_) = pair.find_first_tag("show") {
            return Ok(Self::Show);
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::EOS, _span))
    }
}
#[automatically_derived]
impl FromStr for EosNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::EOS)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for EosFreeNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for EosFreeNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::EOS_FREE)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for DefineNamespaceNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            // Missing rule NamepathFree
            op_namespace: pair.take_tagged_option::<OpNamespaceNode>(Cow::Borrowed("op_namespace")),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for DefineNamespaceNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::DefineNamespace)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwNamespaceNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for KwNamespaceNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_NAMESPACE)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for OpNamespaceNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Hide => None,
            Self::Main => None,
            Self::Test => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("hide") {
            return Ok(Self::Hide);
        }
        if let Some(_) = pair.find_first_tag("main") {
            return Ok(Self::Main);
        }
        if let Some(_) = pair.find_first_tag("test") {
            return Ok(Self::Test);
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::OP_NAMESPACE, _span))
    }
}
#[automatically_derived]
impl FromStr for OpNamespaceNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::OP_NAMESPACE)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for DefineImportNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            import_term: pair
                .take_tagged_items::<ImportTermNode>(Cow::Borrowed("import_term"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for DefineImportNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::DefineImport)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ImportTermNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::ImportAll(s) => s.get_range(),
            Self::ImportAs(s) => s.get_range(),
            Self::ImportBlock(s) => s.get_range(),
            Self::ImportMacro(s) => s.get_range(),
            Self::NamepathFree(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<ImportAllNode>(Cow::Borrowed("import_all")) {
            return Ok(Self::ImportAll(s));
        }
        if let Ok(s) = pair.take_tagged_one::<ImportAsNode>(Cow::Borrowed("import_as")) {
            return Ok(Self::ImportAs(s));
        }
        if let Ok(s) = pair.take_tagged_one::<ImportBlockNode>(Cow::Borrowed("import_block")) {
            return Ok(Self::ImportBlock(s));
        }
        if let Ok(s) = pair.take_tagged_one::<ImportMacroNode>(Cow::Borrowed("import_macro")) {
            return Ok(Self::ImportMacro(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("namepath_free")) {
            return Ok(Self::NamepathFree(s));
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::ImportTerm, _span))
    }
}
#[automatically_derived]
impl FromStr for ImportTermNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ImportTerm)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ImportAsNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            // Missing rule NamepathFree
            // Missing rule Identifier
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ImportAsNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ImportAs)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ImportAllNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            // Missing rule NamepathFree
            op_import_all: pair.take_tagged_one::<OpImportAllNode>(Cow::Borrowed("op_import_all"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ImportAllNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ImportAll)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ImportBlockNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            import_term: pair
                .take_tagged_items::<ImportTermNode>(Cow::Borrowed("import_term"))
                .collect::<Result<Vec<_>, _>>()?,
            // Missing rule NamepathFree
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ImportBlockNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ImportBlock)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ImportMacroNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            import_macro_item: pair.take_tagged_one::<ImportMacroItemNode>(Cow::Borrowed("import_macro_item"))?,
            // Missing rule NamepathFree
            alias: pair.take_tagged_one::<ImportMacroItemNode>(Cow::Borrowed("alias"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ImportMacroNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ImportMacro)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ImportMacroItemNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Capture(s) => s.get_range(),
            Self::Instant(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("capture")) {
            return Ok(Self::Capture(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("instant")) {
            return Ok(Self::Instant(s));
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::ImportMacroItem, _span))
    }
}
#[automatically_derived]
impl FromStr for ImportMacroItemNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ImportMacroItem)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwImportNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for KwImportNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_IMPORT)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for OpImportAllNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for OpImportAllNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::OP_IMPORT_ALL)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for DefineClassNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            class_block: pair.take_tagged_one::<ClassBlockNode>(Cow::Borrowed("class_block"))?,
            class_inherit: pair.take_tagged_option::<ClassInheritNode>(Cow::Borrowed("class_inherit")),
            // Missing rule DefineGeneric
            define_template: pair.take_tagged_option::<DefineTemplateNode>(Cow::Borrowed("define_template")),
            // Missing rule Identifier
            // Missing rule KW_CLASS
            // Missing rule TypeHint
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for DefineClassNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::DefineClass)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ClassBlockNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            class_block_item: pair
                .take_tagged_items::<ClassBlockItemNode>(Cow::Borrowed("class_block_item"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ClassBlockNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ClassBlock)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ClassBlockItemNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::ClassDomain(s) => s.get_range(),
            Self::ClassField(s) => s.get_range(),
            Self::ClassMethod(s) => s.get_range(),
            Self::EosFree(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<ClassDomainNode>(Cow::Borrowed("class_domain")) {
            return Ok(Self::ClassDomain(s));
        }
        if let Ok(s) = pair.take_tagged_one::<ClassFieldNode>(Cow::Borrowed("class_field")) {
            return Ok(Self::ClassField(s));
        }
        if let Ok(s) = pair.take_tagged_one::<ClassMethodNode>(Cow::Borrowed("class_method")) {
            return Ok(Self::ClassMethod(s));
        }
        if let Ok(s) = pair.take_tagged_one::<EosFreeNode>(Cow::Borrowed("eos_free")) {
            return Ok(Self::EosFree(s));
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::ClassBlockItem, _span))
    }
}
#[automatically_derived]
impl FromStr for ClassBlockItemNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ClassBlockItem)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ClassInheritNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            class_inherit_item: pair
                .take_tagged_items::<ClassInheritItemNode>(Cow::Borrowed("class_inherit_item"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ClassInheritNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ClassInherit)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ClassInheritItemNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            // Missing rule Namepath
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ClassInheritItemNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ClassInheritItem)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ClassFieldNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            // Missing rule Identifier
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ClassFieldNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ClassField)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ClassMethodNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            // Missing rule Identifier
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ClassMethodNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ClassMethod)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ClassDomainNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            class_block: pair.take_tagged_one::<ClassBlockNode>(Cow::Borrowed("class_block"))?,
            // Missing rule Identifier
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ClassDomainNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ClassDomain)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for DefineTemplateNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            // Missing rule KW_TEMPLATE
            template_block: pair.take_tagged_one::<TemplateBlockNode>(Cow::Borrowed("template_block"))?,
            template_parameters: pair.take_tagged_option::<TemplateParametersNode>(Cow::Borrowed("template_parameters")),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for DefineTemplateNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::DefineTemplate)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TemplateParametersNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            // Missing rule COMMA
            // Missing rule Identifier
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TemplateParametersNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TemplateParameters)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TemplateBlockNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            eos_free: pair.take_tagged_items::<EosFreeNode>(Cow::Borrowed("eos_free")).collect::<Result<Vec<_>, _>>()?,
            // Missing rule TemplateImplements
            template_statement: pair
                .take_tagged_items::<TemplateStatementNode>(Cow::Borrowed("template_statement"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TemplateBlockNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TemplateBlock)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TemplateStatementNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            // Missing rule WhereBlock
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TemplateStatementNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TemplateStatement)?)
    }
}
