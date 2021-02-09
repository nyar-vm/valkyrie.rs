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
            Self::DefineNamespace(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
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
            eos: pair.take_tagged_option::<EosNode>(Cow::Borrowed("eos")),
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
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
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
            return Ok(Self::False);
        }
        if let Some(_) = pair.find_first_tag("true") {
            return Ok(Self::True);
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
impl YggdrasilNode for OpCategoryNode {
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
impl FromStr for OpCategoryNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::OP_CATEGORY)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwExternalNode {
    type Rule = ValkyrieRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::External => None,
            Self::Inspector => None,
            Self::Parser => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("external") {
            return Ok(Self::External);
        }
        if let Some(_) = pair.find_first_tag("inspector") {
            return Ok(Self::Inspector);
        }
        if let Some(_) = pair.find_first_tag("parser") {
            return Ok(Self::Parser);
        }
        Err(YggdrasilError::invalid_node(ValkyrieRule::KW_EXTERNAL, _span))
    }
}
#[automatically_derived]
impl FromStr for KwExternalNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_EXTERNAL)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwGrammarNode {
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
impl FromStr for KwGrammarNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_GRAMMAR)?)
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
impl YggdrasilNode for KwClassNode {
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
impl FromStr for KwClassNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_CLASS)?)
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
impl YggdrasilNode for KwGroupNode {
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
impl FromStr for KwGroupNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_GROUP)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwClimbNode {
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
impl FromStr for KwClimbNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_CLIMB)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for KwMacroNode {
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
impl FromStr for KwMacroNode {
    type Err = YggdrasilError<ValkyrieRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<ValkyrieRule>> {
        Self::from_cst(ValkyrieParser::parse_cst(input, ValkyrieRule::KW_MACRO)?)
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
