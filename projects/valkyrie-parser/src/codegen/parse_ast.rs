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
            Self::DefineClass(s) => s.get_range(),
            Self::DefineImport(s) => s.get_range(),
            Self::DefineNamespace(s) => s.get_range(),
            Self::DefineTrait(s) => s.get_range(),
            Self::DefineUnion(s) => s.get_range(),
            Self::MainStatement(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<DefineClassNode>(Cow::Borrowed("define_class")) {
            return Ok(Self::DefineClass(s));
        }
        if let Ok(s) = pair.take_tagged_one::<DefineImportNode>(Cow::Borrowed("define_import")) {
            return Ok(Self::DefineImport(s));
        }
        if let Ok(s) = pair.take_tagged_one::<DefineNamespaceNode>(Cow::Borrowed("define_namespace")) {
            return Ok(Self::DefineNamespace(s));
        }
        if let Ok(s) = pair.take_tagged_one::<DefineTraitNode>(Cow::Borrowed("define_trait")) {
            return Ok(Self::DefineTrait(s));
        }
        if let Ok(s) = pair.take_tagged_one::<DefineUnionNode>(Cow::Borrowed("define_union")) {
            return Ok(Self::DefineUnion(s));
        }
        if let Ok(s) = pair.take_tagged_one::<MainStatementNode>(Cow::Borrowed("main_statement")) {
            return Ok(Self::MainStatement(s));
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
            return Ok(Self::Omit)
        }
        if let Some(_) = pair.find_first_tag("show") {
            return Ok(Self::Show)
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
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
            namepath_free: pair.take_tagged_one::<NamepathFreeNode>(Cow::Borrowed("namepath_free"))?,
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
            return Ok(Self::Hide)
        }
        if let Some(_) = pair.find_first_tag("main") {
            return Ok(Self::Main)
        }
        if let Some(_) = pair.find_first_tag("test") {
            return Ok(Self::Test)
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
            import_term: pair.take_tagged_items::<ImportTermNode>(Cow::Borrowed("import_term")).collect::<Result<Vec<_>, _>>()?,
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
        if let Ok(s) = pair.take_tagged_one::<NamepathFreeNode>(Cow::Borrowed("namepath_free")) {
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
            namepath_free: pair.take_tagged_one::<NamepathFreeNode>(Cow::Borrowed("namepath_free"))?,
            alias: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("alias"))?,
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
            namepath_free: pair.take_tagged_one::<NamepathFreeNode>(Cow::Borrowed("namepath_free"))?,
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
            import_term: pair.take_tagged_items::<ImportTermNode>(Cow::Borrowed("import_term")).collect::<Result<Vec<_>, _>>()?,
            namepath_free: pair.take_tagged_one::<NamepathFreeNode>(Cow::Borrowed("namepath_free"))?,
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
            namepath_free: pair.take_tagged_one::<NamepathFreeNode>(Cow::Borrowed("namepath_free"))?,
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
        if let Ok(s) = pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("capture")) {
            return Ok(Self::Capture(s));
        }
        if let Ok(s) = pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("instant")) {
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
impl YggdrasilNode for DefineClassNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            kw_class: pair.take_tagged_one::<KwClassNode>(Cow::Borrowed("kw_class"))?,
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
            class_block_item: pair.take_tagged_items::<ClassBlockItemNode>(Cow::Borrowed("class_block_item")).collect::<Result<Vec<_>, _>>()?,
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
            class_inherit_item: pair.take_tagged_items::<ClassInheritItemNode>(Cow::Borrowed("class_inherit_item")).collect::<Result<Vec<_>, _>>()?,
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
            namepath: pair.take_tagged_one::<NamepathNode>(Cow::Borrowed("namepath"))?,
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
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
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
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
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
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
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
            kw_template: pair.take_tagged_one::<KwTemplateNode>(Cow::Borrowed("kw_template"))?,
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
            comma: pair.take_tagged_items::<CommaNode>(Cow::Borrowed("comma")).collect::<Result<Vec<_>, _>>()?,
            identifier: pair.take_tagged_items::<IdentifierNode>(Cow::Borrowed("identifier")).collect::<Result<Vec<_>, _>>()?,
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
            template_implements: pair.take_tagged_items::<TemplateImplementsNode>(Cow::Borrowed("template_implements")).collect::<Result<Vec<_>, _>>()?,
            template_statement: pair.take_tagged_items::<TemplateStatementNode>(Cow::Borrowed("template_statement")).collect::<Result<Vec<_>, _>>()?,
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
            where_block: pair.take_tagged_one::<WhereBlockNode>(Cow::Borrowed("where_block"))?,
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
#[automatically_derived]
impl YggdrasilNode for TemplateImplementsNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            kw_implements: pair.take_tagged_one::<KwImplementsNode>(Cow::Borrowed("kw_implements"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TemplateImplementsNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TemplateImplements)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for WhereBlockNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            kw_where: pair.take_tagged_one::<KwWhereNode>(Cow::Borrowed("kw_where"))?,
            where_bound: pair.take_tagged_items::<WhereBoundNode>(Cow::Borrowed("where_bound")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for WhereBlockNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::WhereBlock)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for WhereBoundNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            eos_free: pair.take_tagged_one::<EosFreeNode>(Cow::Borrowed("eos_free"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for WhereBoundNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::WhereBound)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwClassNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwClassNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_CLASS)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for DefineUnionNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            kw_union: pair.take_tagged_one::<KwUnionNode>(Cow::Borrowed("kw_union"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for DefineUnionNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::DefineUnion)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwUnionNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwUnionNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_UNION)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for DefineTraitNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            kw_trait: pair.take_tagged_one::<KwTraitNode>(Cow::Borrowed("kw_trait"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for DefineTraitNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::DefineTrait)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwTraitNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwTraitNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_TRAIT)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for WhileStatementNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            inline_expression: pair.take_tagged_option::<InlineExpressionNode>(Cow::Borrowed("inline_expression")),
            kw_while: pair.take_tagged_one::<KwWhileNode>(Cow::Borrowed("kw_while"))?,
            main_statement: pair.take_tagged_items::<MainStatementNode>(Cow::Borrowed("main_statement")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for WhileStatementNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::WhileStatement)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwWhileNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwWhileNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_WHILE)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ForStatementNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            inline_expression: pair.take_tagged_option::<InlineExpressionNode>(Cow::Borrowed("inline_expression")),
            kw_for: pair.take_tagged_one::<KwForNode>(Cow::Borrowed("kw_for"))?,
            kw_in: pair.take_tagged_one::<KwInNode>(Cow::Borrowed("kw_in"))?,
            main_statement: pair.take_tagged_items::<MainStatementNode>(Cow::Borrowed("main_statement")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ForStatementNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ForStatement)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for MainStatementNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::ForStatement(s) => s.get_range(),
            Self::MainExpression(s) => s.get_range(),
            Self::WhileStatement(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<ForStatementNode>(Cow::Borrowed("for_statement")) {
            return Ok(Self::ForStatement(s));
        }
        if let Ok(s) = pair.take_tagged_one::<MainExpressionNode>(Cow::Borrowed("main_expression")) {
            return Ok(Self::MainExpression(s));
        }
        if let Ok(s) = pair.take_tagged_one::<WhileStatementNode>(Cow::Borrowed("while_statement")) {
            return Ok(Self::WhileStatement(s));
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::MainStatement, _span))
    }
}
#[automatically_derived]
impl FromStr for MainStatementNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::MainStatement)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for MainExpressionNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            eos: pair.take_tagged_option::<EosNode>(Cow::Borrowed("eos")),
            main_infix: pair.take_tagged_items::<MainInfixNode>(Cow::Borrowed("main_infix")).collect::<Result<Vec<_>, _>>()?,
            main_term: pair.take_tagged_items::<MainTermNode>(Cow::Borrowed("main_term")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for MainExpressionNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::MainExpression)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for MainTermNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            main_factor: pair.take_tagged_one::<MainFactorNode>(Cow::Borrowed("main_factor"))?,
            main_prefix: pair.take_tagged_items::<MainPrefixNode>(Cow::Borrowed("main_prefix")).collect::<Result<Vec<_>, _>>()?,
            main_suffix: pair.take_tagged_items::<MainSuffixNode>(Cow::Borrowed("main_suffix")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for MainTermNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::MainTerm)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for MainFactorNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Atomic(s) => s.get_range(),
            Self::MainFactor0(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<AtomicNode>(Cow::Borrowed("atomic")) {
            return Ok(Self::Atomic(s));
        }
        if let Ok(s) = pair.take_tagged_one::<MainExpressionNode>(Cow::Borrowed("main_factor_0")) {
            return Ok(Self::MainFactor0(s));
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::MainFactor, _span))
    }
}
#[automatically_derived]
impl FromStr for MainFactorNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::MainFactor)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for MainInfixNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::And => None,
            Self::Apply2 => None,
            Self::Apply3 => None,
            Self::Contains => None,
            Self::Divide => None,
            Self::DivideAssign => None,
            Self::EE => None,
            Self::EEE => None,
            Self::EQ => None,
            Self::GE => None,
            Self::GEQ => None,
            Self::GG => None,
            Self::GGE => None,
            Self::GGG => None,
            Self::In => None,
            Self::Is(s) => s.get_range(),
            Self::LE => None,
            Self::LEQ => None,
            Self::LL => None,
            Self::LLE => None,
            Self::LLL => None,
            Self::Map => None,
            Self::Minus => None,
            Self::MinusAssign => None,
            Self::Multiply => None,
            Self::MultiplyAssign => None,
            Self::NE => None,
            Self::NEE => None,
            Self::Nand => None,
            Self::Nor => None,
            Self::NotContains => None,
            Self::NotIn => None,
            Self::NotIs => None,
            Self::Or => None,
            Self::Plus => None,
            Self::PlusAssign => None,
            Self::Power => None,
            Self::Remainder => None,
            Self::RemainderAssign => None,
            Self::Surd => None,
            Self::Until => None,
            Self::UpTo => None,
            Self::Xand => None,
            Self::Xor => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("and") {
            return Ok(Self::And)
        }
        if let Some(_) = pair.find_first_tag("apply_2") {
            return Ok(Self::Apply2)
        }
        if let Some(_) = pair.find_first_tag("apply_3") {
            return Ok(Self::Apply3)
        }
        if let Some(_) = pair.find_first_tag("contains") {
            return Ok(Self::Contains)
        }
        if let Some(_) = pair.find_first_tag("divide") {
            return Ok(Self::Divide)
        }
        if let Some(_) = pair.find_first_tag("divide_assign") {
            return Ok(Self::DivideAssign)
        }
        if let Some(_) = pair.find_first_tag("ee") {
            return Ok(Self::EE)
        }
        if let Some(_) = pair.find_first_tag("eee") {
            return Ok(Self::EEE)
        }
        if let Some(_) = pair.find_first_tag("eq") {
            return Ok(Self::EQ)
        }
        if let Some(_) = pair.find_first_tag("ge") {
            return Ok(Self::GE)
        }
        if let Some(_) = pair.find_first_tag("geq") {
            return Ok(Self::GEQ)
        }
        if let Some(_) = pair.find_first_tag("gg") {
            return Ok(Self::GG)
        }
        if let Some(_) = pair.find_first_tag("gge") {
            return Ok(Self::GGE)
        }
        if let Some(_) = pair.find_first_tag("ggg") {
            return Ok(Self::GGG)
        }
        if let Some(_) = pair.find_first_tag("in") {
            return Ok(Self::In)
        }
        if let Ok(s) = pair.take_tagged_one::<KwIsNode>(Cow::Borrowed("is")) {
            return Ok(Self::Is(s));
        }
        if let Some(_) = pair.find_first_tag("le") {
            return Ok(Self::LE)
        }
        if let Some(_) = pair.find_first_tag("leq") {
            return Ok(Self::LEQ)
        }
        if let Some(_) = pair.find_first_tag("ll") {
            return Ok(Self::LL)
        }
        if let Some(_) = pair.find_first_tag("lle") {
            return Ok(Self::LLE)
        }
        if let Some(_) = pair.find_first_tag("lll") {
            return Ok(Self::LLL)
        }
        if let Some(_) = pair.find_first_tag("map") {
            return Ok(Self::Map)
        }
        if let Some(_) = pair.find_first_tag("minus") {
            return Ok(Self::Minus)
        }
        if let Some(_) = pair.find_first_tag("minus_assign") {
            return Ok(Self::MinusAssign)
        }
        if let Some(_) = pair.find_first_tag("multiply") {
            return Ok(Self::Multiply)
        }
        if let Some(_) = pair.find_first_tag("multiply_assign") {
            return Ok(Self::MultiplyAssign)
        }
        if let Some(_) = pair.find_first_tag("ne") {
            return Ok(Self::NE)
        }
        if let Some(_) = pair.find_first_tag("nee") {
            return Ok(Self::NEE)
        }
        if let Some(_) = pair.find_first_tag("nand") {
            return Ok(Self::Nand)
        }
        if let Some(_) = pair.find_first_tag("nor") {
            return Ok(Self::Nor)
        }
        if let Some(_) = pair.find_first_tag("not_contains") {
            return Ok(Self::NotContains)
        }
        if let Some(_) = pair.find_first_tag("not_in") {
            return Ok(Self::NotIn)
        }
        if let Some(_) = pair.find_first_tag("not_is") {
            return Ok(Self::NotIs)
        }
        if let Some(_) = pair.find_first_tag("or") {
            return Ok(Self::Or)
        }
        if let Some(_) = pair.find_first_tag("plus") {
            return Ok(Self::Plus)
        }
        if let Some(_) = pair.find_first_tag("plus_assign") {
            return Ok(Self::PlusAssign)
        }
        if let Some(_) = pair.find_first_tag("power") {
            return Ok(Self::Power)
        }
        if let Some(_) = pair.find_first_tag("remainder") {
            return Ok(Self::Remainder)
        }
        if let Some(_) = pair.find_first_tag("remainder_assign") {
            return Ok(Self::RemainderAssign)
        }
        if let Some(_) = pair.find_first_tag("surd") {
            return Ok(Self::Surd)
        }
        if let Some(_) = pair.find_first_tag("until") {
            return Ok(Self::Until)
        }
        if let Some(_) = pair.find_first_tag("up_to") {
            return Ok(Self::UpTo)
        }
        if let Some(_) = pair.find_first_tag("xand") {
            return Ok(Self::Xand)
        }
        if let Some(_) = pair.find_first_tag("xor") {
            return Ok(Self::Xor)
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::MainInfix, _span))
    }
}
#[automatically_derived]
impl FromStr for MainInfixNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::MainInfix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for MainPrefixNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Deconstruct => None,
            Self::DeconstructAll => None,
            Self::Dereference => None,
            Self::Inverse => None,
            Self::Negative => None,
            Self::Not => None,
            Self::Positive => None,
            Self::Reference => None,
            Self::Root2 => None,
            Self::Root3 => None,
            Self::Root4 => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("deconstruct") {
            return Ok(Self::Deconstruct)
        }
        if let Some(_) = pair.find_first_tag("deconstruct_all") {
            return Ok(Self::DeconstructAll)
        }
        if let Some(_) = pair.find_first_tag("dereference") {
            return Ok(Self::Dereference)
        }
        if let Some(_) = pair.find_first_tag("inverse") {
            return Ok(Self::Inverse)
        }
        if let Some(_) = pair.find_first_tag("negative") {
            return Ok(Self::Negative)
        }
        if let Some(_) = pair.find_first_tag("not") {
            return Ok(Self::Not)
        }
        if let Some(_) = pair.find_first_tag("positive") {
            return Ok(Self::Positive)
        }
        if let Some(_) = pair.find_first_tag("reference") {
            return Ok(Self::Reference)
        }
        if let Some(_) = pair.find_first_tag("root_2") {
            return Ok(Self::Root2)
        }
        if let Some(_) = pair.find_first_tag("root_3") {
            return Ok(Self::Root3)
        }
        if let Some(_) = pair.find_first_tag("root_4") {
            return Ok(Self::Root4)
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::MainPrefix, _span))
    }
}
#[automatically_derived]
impl FromStr for MainPrefixNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::MainPrefix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for MainSuffixNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Celsius => None,
            Self::Fahrenheit => None,
            Self::Percent2 => None,
            Self::Percent3 => None,
            Self::Percent4 => None,
            Self::Prime1 => None,
            Self::Prime2 => None,
            Self::Prime3 => None,
            Self::Prime4 => None,
            Self::Raise => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("celsius") {
            return Ok(Self::Celsius)
        }
        if let Some(_) = pair.find_first_tag("fahrenheit") {
            return Ok(Self::Fahrenheit)
        }
        if let Some(_) = pair.find_first_tag("percent_2") {
            return Ok(Self::Percent2)
        }
        if let Some(_) = pair.find_first_tag("percent_3") {
            return Ok(Self::Percent3)
        }
        if let Some(_) = pair.find_first_tag("percent_4") {
            return Ok(Self::Percent4)
        }
        if let Some(_) = pair.find_first_tag("prime_1") {
            return Ok(Self::Prime1)
        }
        if let Some(_) = pair.find_first_tag("prime_2") {
            return Ok(Self::Prime2)
        }
        if let Some(_) = pair.find_first_tag("prime_3") {
            return Ok(Self::Prime3)
        }
        if let Some(_) = pair.find_first_tag("prime_4") {
            return Ok(Self::Prime4)
        }
        if let Some(_) = pair.find_first_tag("raise") {
            return Ok(Self::Raise)
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::MainSuffix, _span))
    }
}
#[automatically_derived]
impl FromStr for MainSuffixNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::MainSuffix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for InlineExpressionNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            inline_term: pair.take_tagged_items::<InlineTermNode>(Cow::Borrowed("inline_term")).collect::<Result<Vec<_>, _>>()?,
            main_infix: pair.take_tagged_items::<MainInfixNode>(Cow::Borrowed("main_infix")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for InlineExpressionNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::InlineExpression)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for InlineTermNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            inline_factor: pair.take_tagged_one::<InlineFactorNode>(Cow::Borrowed("inline_factor"))?,
            main_prefix: pair.take_tagged_items::<MainPrefixNode>(Cow::Borrowed("main_prefix")).collect::<Result<Vec<_>, _>>()?,
            main_suffix: pair.take_tagged_items::<MainSuffixNode>(Cow::Borrowed("main_suffix")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for InlineTermNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::InlineTerm)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for InlineFactorNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Atomic(s) => s.get_range(),
            Self::InlineFactor0(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<AtomicNode>(Cow::Borrowed("atomic")) {
            return Ok(Self::Atomic(s));
        }
        if let Ok(s) = pair.take_tagged_one::<MainExpressionNode>(Cow::Borrowed("inline_factor_0")) {
            return Ok(Self::InlineFactor0(s));
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::InlineFactor, _span))
    }
}
#[automatically_derived]
impl FromStr for InlineFactorNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::InlineFactor)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RangeCallNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            op_and_then: pair.take_tagged_option::<OpAndThenNode>(Cow::Borrowed("op_and_then")),
            range_literal: pair.take_tagged_one::<RangeLiteralNode>(Cow::Borrowed("range_literal"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for RangeCallNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::RangeCall)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RangeLiteralNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            comma: pair.take_tagged_items::<CommaNode>(Cow::Borrowed("comma")).collect::<Result<Vec<_>, _>>()?,
            range_axis: pair.take_tagged_items::<RangeAxisNode>(Cow::Borrowed("range_axis")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for RangeLiteralNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::RangeLiteral)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RangeAxisNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            head: pair.take_tagged_one::<MainExpressionNode>(Cow::Borrowed("head"))?,
            index: pair.take_tagged_one::<MainExpressionNode>(Cow::Borrowed("index"))?,
            step: pair.take_tagged_one::<MainExpressionNode>(Cow::Borrowed("step"))?,
            tail: pair.take_tagged_one::<MainExpressionNode>(Cow::Borrowed("tail"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for RangeAxisNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::RangeAxis)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RangeOmitNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            colon: pair.take_tagged_items::<ColonNode>(Cow::Borrowed("colon")).collect::<Result<Vec<_>, _>>()?,
            proportion: pair.take_tagged_one::<ProportionNode>(Cow::Borrowed("proportion"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for RangeOmitNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::RangeOmit)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for AtomicNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Boolean(s) => s.get_range(),
            Self::Integer(s) => s.get_range(),
            Self::Namepath(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<BooleanNode>(Cow::Borrowed("boolean")) {
            return Ok(Self::Boolean(s));
        }
        if let Ok(s) = pair.take_tagged_one::<IntegerNode>(Cow::Borrowed("integer")) {
            return Ok(Self::Integer(s));
        }
        if let Ok(s) = pair.take_tagged_one::<NamepathNode>(Cow::Borrowed("namepath")) {
            return Ok(Self::Namepath(s));
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::Atomic, _span))
    }
}
#[automatically_derived]
impl FromStr for AtomicNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::Atomic)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for NamepathFreeNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_items::<IdentifierNode>(Cow::Borrowed("identifier")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for NamepathFreeNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::NamepathFree)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for NamepathNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_items::<IdentifierNode>(Cow::Borrowed("identifier")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for NamepathNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::Namepath)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IdentifierNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::IdentifierBare(s) => s.get_range(),
            Self::IdentifierRaw(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<IdentifierBareNode>(Cow::Borrowed("identifier_bare")) {
            return Ok(Self::IdentifierBare(s));
        }
        if let Ok(s) = pair.take_tagged_one::<IdentifierRawNode>(Cow::Borrowed("identifier_raw")) {
            return Ok(Self::IdentifierRaw(s));
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::Identifier, _span))
    }
}
#[automatically_derived]
impl FromStr for IdentifierNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::Identifier)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IdentifierBareNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            text: pair.get_string(),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for IdentifierBareNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::IdentifierBare)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IdentifierRawNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier_raw_text: pair.take_tagged_one::<IdentifierRawTextNode>(Cow::Borrowed("identifier_raw_text"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for IdentifierRawNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::IdentifierRaw)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IdentifierRawTextNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            text: pair.get_string(),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for IdentifierRawTextNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::IdentifierRawText)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for BooleanNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::False => None,
            Self::True => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("false") {
            return Ok(Self::False)
        }
        if let Some(_) = pair.find_first_tag("true") {
            return Ok(Self::True)
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::Boolean, _span))
    }
}
#[automatically_derived]
impl FromStr for BooleanNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::Boolean)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IntegerNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            text: pair.get_string(),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for IntegerNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::Integer)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RangeExactNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            integer: pair.take_tagged_one::<IntegerNode>(Cow::Borrowed("integer"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for RangeExactNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::RangeExact)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RangeNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            max: pair.take_tagged_option::<IntegerNode>(Cow::Borrowed("max")),
            min: pair.take_tagged_option::<IntegerNode>(Cow::Borrowed("min")),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for RangeNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::Range)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ModifierCallNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ModifierCallNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ModifierCall)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for CommaNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for CommaNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::COMMA)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ColonNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ColonNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::COLON)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ProportionNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ProportionNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::PROPORTION)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for DotNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for DotNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::DOT)?)
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
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
impl YggdrasilNode for OpAndThenNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for OpAndThenNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::OP_AND_THEN)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for OpBindNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for OpBindNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::OP_BIND)?)
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
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
impl YggdrasilNode for KwImportNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
impl YggdrasilNode for KwTemplateNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwTemplateNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_TEMPLATE)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwWhereNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwWhereNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_WHERE)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwImplementsNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwImplementsNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_IMPLEMENTS)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwExtendsNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwExtendsNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_EXTENDS)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwInheritsNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwInheritsNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_INHERITS)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwIfNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwIfNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_IF)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwElseNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwElseNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_ELSE)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwForNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwForNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_FOR)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwReturnNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwReturnNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_RETURN)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwBreakNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwBreakNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_BREAK)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwContinueNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwContinueNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_CONTINUE)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwNotNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwNotNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_NOT)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwInNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwInNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_IN)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwIsNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwIsNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_IS)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwAsNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for KwAsNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_AS)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for WhiteSpaceNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for WhiteSpaceNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::WhiteSpace)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for CommentNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for CommentNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::Comment)?)
    }
}
