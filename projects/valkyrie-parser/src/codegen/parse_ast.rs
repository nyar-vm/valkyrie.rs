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
            Self::DefineEnumerate(s) => s.get_range(),
            Self::DefineFunction(s) => s.get_range(),
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
        if let Ok(s) = pair.take_tagged_one::<DefineEnumerateNode>(Cow::Borrowed("define_enumerate")) {
            return Ok(Self::DefineEnumerate(s));
        }
        if let Ok(s) = pair.take_tagged_one::<DefineFunctionNode>(Cow::Borrowed("define_function")) {
            return Ok(Self::DefineFunction(s));
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
            import_term: pair
                .take_tagged_items::<ImportTermNode>(Cow::Borrowed("import_term"))
                .collect::<Result<Vec<_>, _>>()?,
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
impl YggdrasilNode for DefineTemplateNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            attribute_call: pair
                .take_tagged_items::<AttributeCallNode>(Cow::Borrowed("attribute_call"))
                .collect::<Result<Vec<_>, _>>()?,
            kw_template: pair.take_tagged_one::<KwTemplateNode>(Cow::Borrowed("kw_template"))?,
            modifier_call: pair
                .take_tagged_items::<ModifierCallNode>(Cow::Borrowed("modifier_call"))
                .collect::<Result<Vec<_>, _>>()?,
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
            template_implements: pair
                .take_tagged_items::<TemplateImplementsNode>(Cow::Borrowed("template_implements"))
                .collect::<Result<Vec<_>, _>>()?,
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
            where_bound: pair
                .take_tagged_items::<WhereBoundNode>(Cow::Borrowed("where_bound"))
                .collect::<Result<Vec<_>, _>>()?,
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
impl YggdrasilNode for DefineClassNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            attribute_call: pair
                .take_tagged_items::<AttributeCallNode>(Cow::Borrowed("attribute_call"))
                .collect::<Result<Vec<_>, _>>()?,
            class_block: pair.take_tagged_one::<ClassBlockNode>(Cow::Borrowed("class_block"))?,
            class_inherit: pair.take_tagged_option::<ClassInheritNode>(Cow::Borrowed("class_inherit")),
            define_template: pair.take_tagged_option::<DefineTemplateNode>(Cow::Borrowed("define_template")),
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            kw_class: pair.take_tagged_one::<KwClassNode>(Cow::Borrowed("kw_class"))?,
            modifier_call: pair
                .take_tagged_items::<ModifierCallNode>(Cow::Borrowed("modifier_call"))
                .collect::<Result<Vec<_>, _>>()?,
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
            attribute_call: pair
                .take_tagged_items::<AttributeCallNode>(Cow::Borrowed("attribute_call"))
                .collect::<Result<Vec<_>, _>>()?,
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            parameter_default: pair.take_tagged_option::<ParameterDefaultNode>(Cow::Borrowed("parameter_default")),
            type_hint: pair.take_tagged_option::<TypeHintNode>(Cow::Borrowed("type_hint")),
            field_modifier: pair
                .take_tagged_items::<FieldModifierNode>(Cow::Borrowed("field_modifier"))
                .collect::<Result<Vec<_>, _>>()?,
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
impl YggdrasilNode for FieldModifierNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            modifier_call: pair.take_tagged_one::<ModifierCallNode>(Cow::Borrowed("modifier_call"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for FieldModifierNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::field_modifier)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ParameterDefaultNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            main_expression: pair.take_tagged_one::<MainExpressionNode>(Cow::Borrowed("main_expression"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ParameterDefaultNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ParameterDefault)?)
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
            attribute_call: pair
                .take_tagged_items::<AttributeCallNode>(Cow::Borrowed("attribute_call"))
                .collect::<Result<Vec<_>, _>>()?,
            namepath: pair.take_tagged_one::<NamepathNode>(Cow::Borrowed("namepath"))?,
            method_modifier: pair
                .take_tagged_items::<MethodModifierNode>(Cow::Borrowed("method_modifier"))
                .collect::<Result<Vec<_>, _>>()?,
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
impl YggdrasilNode for MethodModifierNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            modifier_call: pair.take_tagged_one::<ModifierCallNode>(Cow::Borrowed("modifier_call"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for MethodModifierNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::method_modifier)?)
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
            attribute_call: pair
                .take_tagged_items::<AttributeCallNode>(Cow::Borrowed("attribute_call"))
                .collect::<Result<Vec<_>, _>>()?,
            class_block: pair.take_tagged_one::<ClassBlockNode>(Cow::Borrowed("class_block"))?,
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            field_modifier: pair
                .take_tagged_items::<FieldModifierNode>(Cow::Borrowed("field_modifier"))
                .collect::<Result<Vec<_>, _>>()?,
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
impl YggdrasilNode for KwClassNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Class => None,
            Self::Structure => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("class") {
            return Ok(Self::Class);
        }
        if let Some(_) = pair.find_first_tag("structure") {
            return Ok(Self::Structure);
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::KW_CLASS, _span))
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
            attribute_call: pair
                .take_tagged_items::<AttributeCallNode>(Cow::Borrowed("attribute_call"))
                .collect::<Result<Vec<_>, _>>()?,
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
impl YggdrasilNode for DefineEnumerateNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            attribute_call: pair
                .take_tagged_items::<AttributeCallNode>(Cow::Borrowed("attribute_call"))
                .collect::<Result<Vec<_>, _>>()?,
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            kw_flags: pair.take_tagged_one::<KwFlagsNode>(Cow::Borrowed("kw_flags"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for DefineEnumerateNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::DefineEnumerate)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwFlagsNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Enum => None,
            Self::Flags => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("enum") {
            return Ok(Self::Enum);
        }
        if let Some(_) = pair.find_first_tag("flags") {
            return Ok(Self::Flags);
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::KW_FLAGS, _span))
    }
}
#[automatically_derived]
impl FromStr for KwFlagsNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_FLAGS)?)
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
impl YggdrasilNode for DefineFunctionNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            kw_function: pair.take_tagged_one::<KwFunctionNode>(Cow::Borrowed("kw_function"))?,
            namepath: pair.take_tagged_one::<NamepathNode>(Cow::Borrowed("namepath"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for DefineFunctionNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::DefineFunction)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwFunctionNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Macro => None,
            Self::Micro => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("macro") {
            return Ok(Self::Macro);
        }
        if let Some(_) = pair.find_first_tag("micro") {
            return Ok(Self::Micro);
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::KW_FUNCTION, _span))
    }
}
#[automatically_derived]
impl FromStr for KwFunctionNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_FUNCTION)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ContinuationNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            main_statement: pair
                .take_tagged_items::<MainStatementNode>(Cow::Borrowed("main_statement"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ContinuationNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::Continuation)?)
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
            continuation: pair.take_tagged_one::<ContinuationNode>(Cow::Borrowed("continuation"))?,
            inline_expression: pair.take_tagged_option::<InlineExpressionNode>(Cow::Borrowed("inline_expression")),
            kw_while: pair.take_tagged_one::<KwWhileNode>(Cow::Borrowed("kw_while"))?,
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
        match self {
            Self::Until => None,
            Self::While => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("until") {
            return Ok(Self::Until);
        }
        if let Some(_) = pair.find_first_tag("while") {
            return Ok(Self::While);
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::KW_WHILE, _span))
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
            continuation: pair.take_tagged_one::<ContinuationNode>(Cow::Borrowed("continuation"))?,
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            inline_expression: pair.take_tagged_option::<InlineExpressionNode>(Cow::Borrowed("inline_expression")),
            kw_for: pair.take_tagged_one::<KwForNode>(Cow::Borrowed("kw_for"))?,
            kw_in: pair.take_tagged_one::<KwInNode>(Cow::Borrowed("kw_in"))?,
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
            Self::ExpressionStatement(s) => s.get_range(),
            Self::ForStatement(s) => s.get_range(),
            Self::WhileStatement(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<ExpressionStatementNode>(Cow::Borrowed("expression_statement")) {
            return Ok(Self::ExpressionStatement(s));
        }
        if let Ok(s) = pair.take_tagged_one::<ForStatementNode>(Cow::Borrowed("for_statement")) {
            return Ok(Self::ForStatement(s));
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
impl YggdrasilNode for ExpressionStatementNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            eos: pair.take_tagged_option::<EosNode>(Cow::Borrowed("eos")),
            main_expression: pair.take_tagged_one::<MainExpressionNode>(Cow::Borrowed("main_expression"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ExpressionStatementNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ExpressionStatement)?)
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
            main_prefix: pair
                .take_tagged_items::<MainPrefixNode>(Cow::Borrowed("main_prefix"))
                .collect::<Result<Vec<_>, _>>()?,
            main_suffix: pair
                .take_tagged_items::<MainSuffixNode>(Cow::Borrowed("main_suffix"))
                .collect::<Result<Vec<_>, _>>()?,
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
            Self::GroupFactor(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<AtomicNode>(Cow::Borrowed("atomic")) {
            return Ok(Self::Atomic(s));
        }
        if let Ok(s) = pair.take_tagged_one::<GroupFactorNode>(Cow::Borrowed("group_factor")) {
            return Ok(Self::GroupFactor(s));
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
impl YggdrasilNode for GroupFactorNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            main_expression: pair.take_tagged_one::<MainExpressionNode>(Cow::Borrowed("main_expression"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for GroupFactorNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::GroupFactor)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for AtomicNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Integer(s) => s.get_range(),
            Self::Namepath(s) => s.get_range(),
            Self::ProceduralCall(s) => s.get_range(),
            Self::RangeLiteral(s) => s.get_range(),
            Self::Special(s) => s.get_range(),
            Self::TextLiteral(s) => s.get_range(),
            Self::TupleLiteral(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<IntegerNode>(Cow::Borrowed("integer")) {
            return Ok(Self::Integer(s));
        }
        if let Ok(s) = pair.take_tagged_one::<NamepathNode>(Cow::Borrowed("namepath")) {
            return Ok(Self::Namepath(s));
        }
        if let Ok(s) = pair.take_tagged_one::<ProceduralCallNode>(Cow::Borrowed("procedural_call")) {
            return Ok(Self::ProceduralCall(s));
        }
        if let Ok(s) = pair.take_tagged_one::<RangeLiteralNode>(Cow::Borrowed("range_literal")) {
            return Ok(Self::RangeLiteral(s));
        }
        if let Ok(s) = pair.take_tagged_one::<SpecialNode>(Cow::Borrowed("special")) {
            return Ok(Self::Special(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TextLiteralNode>(Cow::Borrowed("text_literal")) {
            return Ok(Self::TextLiteral(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TupleLiteralNode>(Cow::Borrowed("tuple_literal")) {
            return Ok(Self::TupleLiteral(s));
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
impl YggdrasilNode for MainInfixNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
            Self::InlineSuffix(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<InlineSuffixNode>(Cow::Borrowed("inline_suffix")) {
            return Ok(Self::InlineSuffix(s));
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
            inline_term: pair
                .take_tagged_items::<InlineTermNode>(Cow::Borrowed("inline_term"))
                .collect::<Result<Vec<_>, _>>()?,
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
            inline_suffix: pair
                .take_tagged_items::<InlineSuffixNode>(Cow::Borrowed("inline_suffix"))
                .collect::<Result<Vec<_>, _>>()?,
            main_factor: pair.take_tagged_one::<MainFactorNode>(Cow::Borrowed("main_factor"))?,
            main_prefix: pair
                .take_tagged_items::<MainPrefixNode>(Cow::Borrowed("main_prefix"))
                .collect::<Result<Vec<_>, _>>()?,
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
impl YggdrasilNode for InlineSuffixNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::DotCall(s) => s.get_range(),
            Self::InlineSuffix0(s) => s.get_range(),
            Self::RangeCall(s) => s.get_range(),
            Self::TupleCall(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<DotCallNode>(Cow::Borrowed("dot_call")) {
            return Ok(Self::DotCall(s));
        }
        if let Ok(s) = pair.take_tagged_one::<SuffixOperatorNode>(Cow::Borrowed("inline_suffix_0")) {
            return Ok(Self::InlineSuffix0(s));
        }
        if let Ok(s) = pair.take_tagged_one::<RangeCallNode>(Cow::Borrowed("range_call")) {
            return Ok(Self::RangeCall(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TupleCallNode>(Cow::Borrowed("tuple_call")) {
            return Ok(Self::TupleCall(s));
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::InlineSuffix, _span))
    }
}
#[automatically_derived]
impl FromStr for InlineSuffixNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::InlineSuffix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for SuffixOperatorNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for SuffixOperatorNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::SuffixOperator)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TypeHintNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            colon: pair.take_tagged_one::<ColonNode>(Cow::Borrowed("colon"))?,
            type_expression: pair.take_tagged_one::<TypeExpressionNode>(Cow::Borrowed("type_expression"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TypeHintNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TypeHint)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TypeExpressionNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            type_infix: pair.take_tagged_items::<TypeInfixNode>(Cow::Borrowed("type_infix")).collect::<Result<Vec<_>, _>>()?,
            type_term: pair.take_tagged_items::<TypeTermNode>(Cow::Borrowed("type_term")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TypeExpressionNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TypeExpression)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TypeTermNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            main_factor: pair.take_tagged_one::<MainFactorNode>(Cow::Borrowed("main_factor"))?,
            type_prefix: pair
                .take_tagged_items::<TypePrefixNode>(Cow::Borrowed("type_prefix"))
                .collect::<Result<Vec<_>, _>>()?,
            type_suffix: pair
                .take_tagged_items::<TypeSuffixNode>(Cow::Borrowed("type_suffix"))
                .collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TypeTermNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TypeTerm)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TypeFactorNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Atomic(s) => s.get_range(),
            Self::TypeFactor0(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<AtomicNode>(Cow::Borrowed("atomic")) {
            return Ok(Self::Atomic(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TypeExpressionNode>(Cow::Borrowed("type_factor_0")) {
            return Ok(Self::TypeFactor0(s));
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::TypeFactor, _span))
    }
}
#[automatically_derived]
impl FromStr for TypeFactorNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TypeFactor)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TypeInfixNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Union => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("union") {
            return Ok(Self::Union);
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::TypeInfix, _span))
    }
}
#[automatically_derived]
impl FromStr for TypeInfixNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TypeInfix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TypePrefixNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Negative => None,
            Self::Positive => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("negative") {
            return Ok(Self::Negative);
        }
        if let Some(_) = pair.find_first_tag("positive") {
            return Ok(Self::Positive);
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::TypePrefix, _span))
    }
}
#[automatically_derived]
impl FromStr for TypePrefixNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TypePrefix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TypeSuffixNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Option => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("option") {
            return Ok(Self::Option);
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::TypeSuffix, _span))
    }
}
#[automatically_derived]
impl FromStr for TypeSuffixNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TypeSuffix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for DotCallNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            dot_call_item: pair.take_tagged_one::<DotCallItemNode>(Cow::Borrowed("dot_call_item"))?,
            op_and_then: pair.take_tagged_option::<OpAndThenNode>(Cow::Borrowed("op_and_then")),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for DotCallNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::DotCall)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for DotCallItemNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Integer(s) => s.get_range(),
            Self::Namepath(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<IntegerNode>(Cow::Borrowed("integer")) {
            return Ok(Self::Integer(s));
        }
        if let Ok(s) = pair.take_tagged_one::<NamepathNode>(Cow::Borrowed("namepath")) {
            return Ok(Self::Namepath(s));
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::DotCallItem, _span))
    }
}
#[automatically_derived]
impl FromStr for DotCallItemNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::DotCallItem)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TupleCallNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            continuation: pair.take_tagged_option::<ContinuationNode>(Cow::Borrowed("continuation")),
            op_and_then: pair.take_tagged_option::<OpAndThenNode>(Cow::Borrowed("op_and_then")),
            tuple_literal: pair.take_tagged_option::<TupleLiteralNode>(Cow::Borrowed("tuple_literal")),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TupleCallNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TupleCall)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TupleLiteralNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            tuple_pair: pair.take_tagged_items::<TuplePairNode>(Cow::Borrowed("tuple_pair")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TupleLiteralNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TupleLiteral)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TuplePairNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            // Missing rule Colon
            main_expression: pair.take_tagged_one::<MainExpressionNode>(Cow::Borrowed("main_expression"))?,
            tuple_key: pair.take_tagged_option::<TupleKeyNode>(Cow::Borrowed("tuple_key")),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TuplePairNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TuplePair)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TupleKeyNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Identifier(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier")) {
            return Ok(Self::Identifier(s));
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::TupleKey, _span))
    }
}
#[automatically_derived]
impl FromStr for TupleKeyNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TupleKey)?)
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
            subscript_axis: pair
                .take_tagged_items::<SubscriptAxisNode>(Cow::Borrowed("subscript_axis"))
                .collect::<Result<Vec<_>, _>>()?,
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
impl YggdrasilNode for SubscriptAxisNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::SubscriptOnly(s) => s.get_range(),
            Self::SubscriptRange(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<SubscriptOnlyNode>(Cow::Borrowed("subscript_only")) {
            return Ok(Self::SubscriptOnly(s));
        }
        if let Ok(s) = pair.take_tagged_one::<SubscriptRangeNode>(Cow::Borrowed("subscript_range")) {
            return Ok(Self::SubscriptRange(s));
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::SubscriptAxis, _span))
    }
}
#[automatically_derived]
impl FromStr for SubscriptAxisNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::SubscriptAxis)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for SubscriptOnlyNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            index: pair.take_tagged_one::<MainExpressionNode>(Cow::Borrowed("index"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for SubscriptOnlyNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::SubscriptOnly)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for SubscriptRangeNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            head: pair.take_tagged_option::<MainExpressionNode>(Cow::Borrowed("head")),
            step: pair.take_tagged_option::<MainExpressionNode>(Cow::Borrowed("step")),
            tail: pair.take_tagged_option::<MainExpressionNode>(Cow::Borrowed("tail")),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for SubscriptRangeNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::SubscriptRange)?)
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
            proportion: pair.take_tagged_option::<ProportionNode>(Cow::Borrowed("proportion")),
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
impl YggdrasilNode for AttributeCallNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            attribute_path: pair.take_tagged_one::<AttributePathNode>(Cow::Borrowed("attribute_path"))?,
            tuple_literal: pair.take_tagged_option::<TupleLiteralNode>(Cow::Borrowed("tuple_literal")),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for AttributeCallNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::AttributeCall)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ProceduralCallNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            procedural_path: pair.take_tagged_one::<ProceduralPathNode>(Cow::Borrowed("procedural_path"))?,
            tuple_literal: pair.take_tagged_option::<TupleLiteralNode>(Cow::Borrowed("tuple_literal")),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ProceduralCallNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ProceduralCall)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextLiteralNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_option::<IdentifierNode>(Cow::Borrowed("identifier")),
            text_raw: pair.take_tagged_one::<TextRawNode>(Cow::Borrowed("text_raw"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TextLiteralNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TextLiteral)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextRawNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            text_content_1: pair.take_tagged_option::<TextContent1Node>(Cow::Borrowed("text_content_1")),
            text_content_2: pair.take_tagged_option::<TextContent2Node>(Cow::Borrowed("text_content_2")),
            text_content_3: pair.take_tagged_option::<TextContent3Node>(Cow::Borrowed("text_content_3")),
            text_content_4: pair.take_tagged_option::<TextContent4Node>(Cow::Borrowed("text_content_4")),
            text_content_5: pair.take_tagged_option::<TextContent5Node>(Cow::Borrowed("text_content_5")),
            text_content_6: pair.take_tagged_option::<TextContent6Node>(Cow::Borrowed("text_content_6")),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TextRawNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TextRaw)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextContent1Node {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for TextContent1Node {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TEXT_CONTENT1)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextContent2Node {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for TextContent2Node {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TEXT_CONTENT2)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextContent3Node {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for TextContent3Node {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TEXT_CONTENT3)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextContent4Node {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for TextContent4Node {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TEXT_CONTENT4)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextContent5Node {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for TextContent5Node {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TEXT_CONTENT5)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextContent6Node {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for TextContent6Node {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::TEXT_CONTENT6)?)
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
impl YggdrasilNode for AttributePathNode {
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
impl FromStr for AttributePathNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::AttributePath)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ProceduralPathNode {
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
impl FromStr for ProceduralPathNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::ProceduralPath)?)
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
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
impl YggdrasilNode for SpecialNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for SpecialNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::Special)?)
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
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
impl YggdrasilNode for ProportionNode {
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
impl FromStr for ProportionNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::PROPORTION)?)
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
impl YggdrasilNode for CommaNode {
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
impl FromStr for CommaNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::COMMA)?)
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
impl YggdrasilNode for OffsetLNode {
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
impl FromStr for OffsetLNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::OFFSET_L)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for OffsetRNode {
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
impl FromStr for OffsetRNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::OFFSET_R)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for Proportion2Node {
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
impl FromStr for Proportion2Node {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::PROPORTION2)?)
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
impl YggdrasilNode for OpAndThenNode {
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
impl YggdrasilNode for KwTemplateNode {
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
impl YggdrasilNode for SkipSpaceNode {
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
impl FromStr for SkipSpaceNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::SkipSpace)?)
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
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for CommentNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::Comment)?)
    }
}
