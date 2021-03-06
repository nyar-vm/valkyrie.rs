use super::*;

pub(super) fn parse_cst(input: &str, rule: ValkyrieRule) -> OutputResult<ValkyrieRule> {
    state(input, |state| match rule {
        ValkyrieRule::Program => parse_program(state),
        ValkyrieRule::Statement => parse_statement(state),
        ValkyrieRule::EOS => parse_eos(state),
        ValkyrieRule::EOS_FREE => parse_eos_free(state),
        ValkyrieRule::DefineNamespace => parse_define_namespace(state),
        ValkyrieRule::OP_NAMESPACE => parse_op_namespace(state),
        ValkyrieRule::DefineImport => parse_define_import(state),
        ValkyrieRule::ImportTerm => parse_import_term(state),
        ValkyrieRule::ImportAs => parse_import_as(state),
        ValkyrieRule::ImportAll => parse_import_all(state),
        ValkyrieRule::ImportBlock => parse_import_block(state),
        ValkyrieRule::ImportMacro => parse_import_macro(state),
        ValkyrieRule::ImportMacroItem => parse_import_macro_item(state),
        ValkyrieRule::DefineTemplate => parse_define_template(state),
        ValkyrieRule::TemplateParameters => parse_template_parameters(state),
        ValkyrieRule::TemplateBlock => parse_template_block(state),
        ValkyrieRule::TemplateStatement => parse_template_statement(state),
        ValkyrieRule::TemplateImplements => parse_template_implements(state),
        ValkyrieRule::WhereBlock => parse_where_block(state),
        ValkyrieRule::WhereBound => parse_where_bound(state),
        ValkyrieRule::DefineClass => parse_define_class(state),
        ValkyrieRule::ClassBlock => parse_class_block(state),
        ValkyrieRule::ClassBlockItem => parse_class_block_item(state),
        ValkyrieRule::ClassInherit => parse_class_inherit(state),
        ValkyrieRule::ClassInheritItem => parse_class_inherit_item(state),
        ValkyrieRule::ClassField => parse_class_field(state),
        ValkyrieRule::field_modifier => parse_field_modifier(state),
        ValkyrieRule::ParameterDefault => parse_parameter_default(state),
        ValkyrieRule::ClassMethod => parse_class_method(state),
        ValkyrieRule::method_modifier => parse_method_modifier(state),
        ValkyrieRule::ClassDomain => parse_class_domain(state),
        ValkyrieRule::KW_CLASS => parse_kw_class(state),
        ValkyrieRule::DefineUnion => parse_define_union(state),
        ValkyrieRule::KW_UNION => parse_kw_union(state),
        ValkyrieRule::DefineFlags => parse_define_flags(state),
        ValkyrieRule::KW_FLAGS => parse_kw_flags(state),
        ValkyrieRule::DefineTrait => parse_define_trait(state),
        ValkyrieRule::KW_TRAIT => parse_kw_trait(state),
        ValkyrieRule::DefineFunction => parse_define_function(state),
        ValkyrieRule::KW_FUNCTION => parse_kw_function(state),
        ValkyrieRule::WhileStatement => parse_while_statement(state),
        ValkyrieRule::KW_WHILE => parse_kw_while(state),
        ValkyrieRule::ForStatement => parse_for_statement(state),
        ValkyrieRule::MainStatement => parse_main_statement(state),
        ValkyrieRule::MainExpression => parse_main_expression(state),
        ValkyrieRule::MainTerm => parse_main_term(state),
        ValkyrieRule::MainFactor => parse_main_factor(state),
        ValkyrieRule::GroupFactor => parse_group_factor(state),
        ValkyrieRule::Atomic => parse_atomic(state),
        ValkyrieRule::MainInfix => parse_main_infix(state),
        ValkyrieRule::MainPrefix => parse_main_prefix(state),
        ValkyrieRule::MainSuffix => parse_main_suffix(state),
        ValkyrieRule::InlineExpression => parse_inline_expression(state),
        ValkyrieRule::InlineTerm => parse_inline_term(state),
        ValkyrieRule::InlineSuffix => parse_inline_suffix(state),
        ValkyrieRule::SuffixOperator => parse_suffix_operator(state),
        ValkyrieRule::TypeHint => parse_type_hint(state),
        ValkyrieRule::TypeExpression => parse_type_expression(state),
        ValkyrieRule::TypeTerm => parse_type_term(state),
        ValkyrieRule::TypeFactor => parse_type_factor(state),
        ValkyrieRule::TypeInfix => parse_type_infix(state),
        ValkyrieRule::TypePrefix => parse_type_prefix(state),
        ValkyrieRule::TypeSuffix => parse_type_suffix(state),
        ValkyrieRule::TupleCall => parse_tuple_call(state),
        ValkyrieRule::TupleLiteral => parse_tuple_literal(state),
        ValkyrieRule::TuplePair => parse_tuple_pair(state),
        ValkyrieRule::TupleKey => parse_tuple_key(state),
        ValkyrieRule::RangeCall => parse_range_call(state),
        ValkyrieRule::RangeLiteral => parse_range_literal(state),
        ValkyrieRule::SubscriptAxis => parse_subscript_axis(state),
        ValkyrieRule::SubscriptOnly => parse_subscript_only(state),
        ValkyrieRule::SubscriptRange => parse_subscript_range(state),
        ValkyrieRule::RangeOmit => parse_range_omit(state),
        ValkyrieRule::AttributeCall => parse_attribute_call(state),
        ValkyrieRule::ProceduralCall => parse_procedural_call(state),
        ValkyrieRule::ModifierCall => parse_modifier_call(state),
        ValkyrieRule::AttributePath => parse_attribute_path(state),
        ValkyrieRule::ProceduralPath => parse_procedural_path(state),
        ValkyrieRule::NamepathFree => parse_namepath_free(state),
        ValkyrieRule::Namepath => parse_namepath(state),
        ValkyrieRule::Identifier => parse_identifier(state),
        ValkyrieRule::IdentifierBare => parse_identifier_bare(state),
        ValkyrieRule::IdentifierRaw => parse_identifier_raw(state),
        ValkyrieRule::IdentifierRawText => parse_identifier_raw_text(state),
        ValkyrieRule::Special => parse_special(state),
        ValkyrieRule::Integer => parse_integer(state),
        ValkyrieRule::PROPORTION => parse_proportion(state),
        ValkyrieRule::COLON => parse_colon(state),
        ValkyrieRule::COMMA => parse_comma(state),
        ValkyrieRule::DOT => parse_dot(state),
        ValkyrieRule::OFFSET_L => parse_offset_l(state),
        ValkyrieRule::OFFSET_R => parse_offset_r(state),
        ValkyrieRule::PROPORTION2 => parse_proportion_2(state),
        ValkyrieRule::OP_IMPORT_ALL => parse_op_import_all(state),
        ValkyrieRule::OP_AND_THEN => parse_op_and_then(state),
        ValkyrieRule::OP_BIND => parse_op_bind(state),
        ValkyrieRule::KW_NAMESPACE => parse_kw_namespace(state),
        ValkyrieRule::KW_IMPORT => parse_kw_import(state),
        ValkyrieRule::KW_TEMPLATE => parse_kw_template(state),
        ValkyrieRule::KW_WHERE => parse_kw_where(state),
        ValkyrieRule::KW_IMPLEMENTS => parse_kw_implements(state),
        ValkyrieRule::KW_EXTENDS => parse_kw_extends(state),
        ValkyrieRule::KW_INHERITS => parse_kw_inherits(state),
        ValkyrieRule::KW_IF => parse_kw_if(state),
        ValkyrieRule::KW_ELSE => parse_kw_else(state),
        ValkyrieRule::KW_FOR => parse_kw_for(state),
        ValkyrieRule::KW_RETURN => parse_kw_return(state),
        ValkyrieRule::KW_BREAK => parse_kw_break(state),
        ValkyrieRule::KW_CONTINUE => parse_kw_continue(state),
        ValkyrieRule::KW_NOT => parse_kw_not(state),
        ValkyrieRule::KW_IN => parse_kw_in(state),
        ValkyrieRule::KW_IS => parse_kw_is(state),
        ValkyrieRule::KW_AS => parse_kw_as(state),
        ValkyrieRule::WhiteSpace => parse_white_space(state),
        ValkyrieRule::Comment => parse_comment(state),
        ValkyrieRule::HiddenText => unreachable!(),
    })
}
#[inline]
fn parse_program(state: Input) -> Output {
    state.rule(ValkyrieRule::Program, |s| {
        s.repeat(0..4294967295, |s| {
            s.sequence(|s| {
                Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| parse_statement(s).and_then(|s| s.tag_node("statement")))
            })
        })
    })
}
#[inline]
fn parse_statement(state: Input) -> Output {
    state.rule(ValkyrieRule::Statement, |s| {
        Err(s)
            .or_else(|s| parse_define_namespace(s).and_then(|s| s.tag_node("define_namespace")))
            .or_else(|s| parse_define_import(s).and_then(|s| s.tag_node("define_import")))
            .or_else(|s| parse_define_class(s).and_then(|s| s.tag_node("define_class")))
            .or_else(|s| parse_define_union(s).and_then(|s| s.tag_node("define_union")))
            .or_else(|s| parse_define_flags(s).and_then(|s| s.tag_node("define_flags")))
            .or_else(|s| parse_define_trait(s).and_then(|s| s.tag_node("define_trait")))
            .or_else(|s| parse_define_function(s).and_then(|s| s.tag_node("define_function")))
            .or_else(|s| parse_main_statement(s).and_then(|s| s.tag_node("main_statement")))
    })
}
#[inline]
fn parse_eos(state: Input) -> Output {
    state.rule(ValkyrieRule::EOS, |s| {
        Err(s)
            .or_else(|s| {
                builtin_regex(s, {
                    static REGEX: OnceLock<Regex> = OnceLock::new();
                    REGEX.get_or_init(|| Regex::new("^(?x)([;；])").unwrap())
                })
                .and_then(|s| s.tag_node("omit"))
            })
            .or_else(|s| {
                builtin_regex(s, {
                    static REGEX: OnceLock<Regex> = OnceLock::new();
                    REGEX.get_or_init(|| Regex::new("^(?x)(⁏|;;)").unwrap())
                })
                .and_then(|s| s.tag_node("show"))
            })
    })
}
#[inline]
fn parse_eos_free(state: Input) -> Output {
    state.rule(ValkyrieRule::EOS_FREE, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([,，;；⁏])").unwrap())
        })
    })
}
#[inline]
fn parse_define_namespace(state: Input) -> Output {
    state.rule(ValkyrieRule::DefineNamespace, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_kw_namespace(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_op_namespace(s).and_then(|s| s.tag_node("op_namespace"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_namepath_free(s).and_then(|s| s.tag_node("namepath_free")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_eos(s)))
        })
    })
}
#[inline]
fn parse_op_namespace(state: Input) -> Output {
    state.rule(ValkyrieRule::OP_NAMESPACE, |s| {
        Err(s)
            .or_else(|s| builtin_text(s, "!", false).and_then(|s| s.tag_node("main")))
            .or_else(|s| builtin_text(s, "?", false).and_then(|s| s.tag_node("test")))
            .or_else(|s| builtin_text(s, "*", false).and_then(|s| s.tag_node("hide")))
    })
}
#[inline]
fn parse_define_import(state: Input) -> Output {
    state.rule(ValkyrieRule::DefineImport, |s| {
        Err(s)
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| parse_kw_import(s))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| s.optional(|s| parse_import_term(s).and_then(|s| s.tag_node("import_term"))))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| s.optional(|s| parse_eos(s)))
                })
            })
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| parse_kw_import(s))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| builtin_text(s, "{", false))
                        .and_then(|s| {
                            s.repeat(0..4294967295, |s| {
                                s.sequence(|s| {
                                    Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                        Err(s)
                                            .or_else(|s| parse_import_term(s).and_then(|s| s.tag_node("import_term")))
                                            .or_else(|s| parse_eos_free(s))
                                    })
                                })
                            })
                        })
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| builtin_text(s, "}", false))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| s.optional(|s| parse_eos(s)))
                })
            })
    })
}
#[inline]
fn parse_import_term(state: Input) -> Output {
    state.rule(ValkyrieRule::ImportTerm, |s| {
        Err(s)
            .or_else(|s| parse_import_as(s).and_then(|s| s.tag_node("import_as")))
            .or_else(|s| parse_import_all(s).and_then(|s| s.tag_node("import_all")))
            .or_else(|s| parse_import_macro(s).and_then(|s| s.tag_node("import_macro")))
            .or_else(|s| parse_import_block(s).and_then(|s| s.tag_node("import_block")))
            .or_else(|s| parse_namepath_free(s).and_then(|s| s.tag_node("namepath_free")))
    })
}
#[inline]
fn parse_import_as(state: Input) -> Output {
    state.rule(ValkyrieRule::ImportAs, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| parse_namepath_free(s).and_then(|s| s.tag_node("namepath_free")))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_kw_as(s))
                            .and_then(|s| builtin_ignore(s))
                    })
                })
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("alias")))
        })
    })
}
#[inline]
fn parse_import_all(state: Input) -> Output {
    state.rule(ValkyrieRule::ImportAll, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_namepath_free(s).and_then(|s| s.tag_node("namepath_free")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    builtin_regex(s, {
                        static REGEX: OnceLock<Regex> = OnceLock::new();
                        REGEX.get_or_init(|| Regex::new("^(?x)([.∷]|::)").unwrap())
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_op_import_all(s).and_then(|s| s.tag_node("op_import_all")))
        })
    })
}
#[inline]
fn parse_import_block(state: Input) -> Output {
    state.rule(ValkyrieRule::ImportBlock, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_namepath_free(s).and_then(|s| s.tag_node("namepath_free")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.optional(|s| {
                        builtin_regex(s, {
                            static REGEX: OnceLock<Regex> = OnceLock::new();
                            REGEX.get_or_init(|| Regex::new("^(?x)([.∷]|::)").unwrap())
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                Err(s)
                                    .or_else(|s| parse_import_term(s).and_then(|s| s.tag_node("import_term")))
                                    .or_else(|s| parse_eos_free(s))
                            })
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_import_macro(state: Input) -> Output {
    state.rule(ValkyrieRule::ImportMacro, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| parse_namepath_free(s).and_then(|s| s.tag_node("namepath_free")))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| {
                                builtin_regex(s, {
                                    static REGEX: OnceLock<Regex> = OnceLock::new();
                                    REGEX.get_or_init(|| Regex::new("^(?x)([.∷]|::)").unwrap())
                                })
                            })
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_import_macro_item(s).and_then(|s| s.tag_node("import_macro_item")))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_kw_as(s))
                            .and_then(|s| builtin_ignore(s))
                    })
                })
                .and_then(|s| parse_import_macro_item(s).and_then(|s| s.tag_node("alias")))
        })
    })
}
#[inline]
fn parse_import_macro_item(state: Input) -> Output {
    state.rule(ValkyrieRule::ImportMacroItem, |s| {
        Err(s)
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_text(s, "#", false))
                        .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                })
                .and_then(|s| s.tag_node("capture"))
            })
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_text(s, "@", false))
                        .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                })
                .and_then(|s| s.tag_node("instant"))
            })
    })
}
#[inline]
fn parse_define_template(state: Input) -> Output {
    state.rule(ValkyrieRule::DefineTemplate, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_attribute_call(s).and_then(|s| s.tag_node("attribute_call")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_modifier_call(s).and_then(|s| s.tag_node("modifier_call")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_kw_template(s).and_then(|s| s.tag_node("kw_template")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_template_parameters(s).and_then(|s| s.tag_node("template_parameters"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_template_block(s).and_then(|s| s.tag_node("template_block")))
        })
    })
}
#[inline]
fn parse_template_parameters(state: Input) -> Output {
    state.rule(ValkyrieRule::TemplateParameters, |s| {
        Err(s)
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| {
                            s.repeat(0..4294967295, |s| {
                                s.sequence(|s| {
                                    Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                        s.sequence(|s| {
                                            Ok(s)
                                                .and_then(|s| parse_comma(s))
                                                .and_then(|s| builtin_ignore(s))
                                                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                                        })
                                    })
                                })
                            })
                        })
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| s.optional(|s| parse_comma(s)))
                })
            })
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_text(s, "<", false))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| {
                            s.repeat(0..4294967295, |s| {
                                s.sequence(|s| {
                                    Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                        s.sequence(|s| {
                                            Ok(s)
                                                .and_then(|s| parse_comma(s))
                                                .and_then(|s| builtin_ignore(s))
                                                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                                        })
                                    })
                                })
                            })
                        })
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| s.optional(|s| parse_comma(s)))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| builtin_text(s, ">", false))
                })
            })
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_text(s, "⟨", false))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| {
                            s.repeat(0..4294967295, |s| {
                                s.sequence(|s| {
                                    Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                        s.sequence(|s| {
                                            Ok(s)
                                                .and_then(|s| parse_comma(s))
                                                .and_then(|s| builtin_ignore(s))
                                                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                                        })
                                    })
                                })
                            })
                        })
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| s.optional(|s| parse_comma(s)))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| builtin_text(s, "⟩", false))
                })
            })
    })
}
#[inline]
fn parse_template_block(state: Input) -> Output {
    state.rule(ValkyrieRule::TemplateBlock, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                Err(s)
                                    .or_else(|s| parse_template_statement(s).and_then(|s| s.tag_node("template_statement")))
                                    .or_else(|s| parse_template_implements(s).and_then(|s| s.tag_node("template_implements")))
                                    .or_else(|s| parse_eos_free(s).and_then(|s| s.tag_node("eos_free")))
                            })
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_template_statement(state: Input) -> Output {
    state.rule(ValkyrieRule::TemplateStatement, |s| parse_where_block(s).and_then(|s| s.tag_node("where_block")))
}
#[inline]
fn parse_template_implements(state: Input) -> Output {
    state.rule(ValkyrieRule::TemplateImplements, |s| parse_kw_implements(s).and_then(|s| s.tag_node("kw_implements")))
}
#[inline]
fn parse_where_block(state: Input) -> Output {
    state.rule(ValkyrieRule::WhereBlock, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_kw_where(s).and_then(|s| s.tag_node("kw_where")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_where_bound(s).and_then(|s| s.tag_node("where_bound")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_where_bound(state: Input) -> Output {
    state.rule(ValkyrieRule::WhereBound, |s| parse_eos_free(s).and_then(|s| s.tag_node("eos_free")))
}
#[inline]
fn parse_define_class(state: Input) -> Output {
    state.rule(ValkyrieRule::DefineClass, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| s.optional(|s| parse_define_template(s).and_then(|s| s.tag_node("define_template"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_attribute_call(s).and_then(|s| s.tag_node("attribute_call")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_modifier_call(s).and_then(|s| s.tag_node("modifier_call")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_kw_class(s).and_then(|s| s.tag_node("kw_class")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_class_inherit(s).and_then(|s| s.tag_node("class_inherit"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_class_block(s).and_then(|s| s.tag_node("class_block")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_eos(s)))
        })
    })
}
#[inline]
fn parse_class_block(state: Input) -> Output {
    state.rule(ValkyrieRule::ClassBlock, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_class_block_item(s).and_then(|s| s.tag_node("class_block_item")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_class_block_item(state: Input) -> Output {
    state.rule(ValkyrieRule::ClassBlockItem, |s| {
        Err(s)
            .or_else(|s| parse_class_domain(s).and_then(|s| s.tag_node("class_domain")))
            .or_else(|s| parse_class_method(s).and_then(|s| s.tag_node("class_method")))
            .or_else(|s| parse_class_field(s).and_then(|s| s.tag_node("class_field")))
            .or_else(|s| parse_eos_free(s).and_then(|s| s.tag_node("eos_free")))
    })
}
#[inline]
fn parse_class_inherit(state: Input) -> Output {
    state.rule(ValkyrieRule::ClassInherit, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "(", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| parse_class_inherit_item(s).and_then(|s| s.tag_node("class_inherit_item")))
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| {
                                    s.repeat(0..4294967295, |s| {
                                        s.sequence(|s| {
                                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                                s.sequence(|s| {
                                                    Ok(s)
                                                        .and_then(|s| builtin_text(s, ",", false))
                                                        .and_then(|s| builtin_ignore(s))
                                                        .and_then(|s| {
                                                            parse_class_inherit_item(s)
                                                                .and_then(|s| s.tag_node("class_inherit_item"))
                                                        })
                                                })
                                            })
                                        })
                                    })
                                })
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| s.optional(|s| builtin_text(s, ",", false)))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, ")", false))
        })
    })
}
#[inline]
fn parse_class_inherit_item(state: Input) -> Output {
    state.rule(ValkyrieRule::ClassInheritItem, |s| parse_namepath(s).and_then(|s| s.tag_node("namepath")))
}
#[inline]
fn parse_class_field(state: Input) -> Output {
    state.rule(ValkyrieRule::ClassField, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_attribute_call(s).and_then(|s| s.tag_node("attribute_call")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_field_modifier(s).and_then(|s| s.tag_node("field_modifier")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_type_hint(s).and_then(|s| s.tag_node("type_hint"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_parameter_default(s).and_then(|s| s.tag_node("parameter_default"))))
        })
    })
}
#[inline]
fn parse_field_modifier(state: Input) -> Output {
    state.rule(ValkyrieRule::field_modifier, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.lookahead(false, |s| {
                        s.sequence(|s| {
                            Ok(s).and_then(|s| parse_namepath(s)).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                builtin_regex(s, {
                                    static REGEX: OnceLock<Regex> = OnceLock::new();
                                    REGEX.get_or_init(|| Regex::new("^(?x)([:=};])").unwrap())
                                })
                            })
                        })
                    })
                })
                .and_then(|s| parse_modifier_call(s).and_then(|s| s.tag_node("modifier_call")))
        })
    })
}
#[inline]
fn parse_parameter_default(state: Input) -> Output {
    state.rule(ValkyrieRule::ParameterDefault, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "=", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_main_expression(s).and_then(|s| s.tag_node("main_expression")))
        })
    })
}
#[inline]
fn parse_class_method(state: Input) -> Output {
    state.rule(ValkyrieRule::ClassMethod, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_attribute_call(s).and_then(|s| s.tag_node("attribute_call")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_method_modifier(s).and_then(|s| s.tag_node("method_modifier")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_namepath(s).and_then(|s| s.tag_node("namepath")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "(", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, ")", false))
        })
    })
}
#[inline]
fn parse_method_modifier(state: Input) -> Output {
    state.rule(ValkyrieRule::method_modifier, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.lookahead(false, |s| {
                        s.sequence(|s| {
                            Ok(s).and_then(|s| parse_namepath(s)).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                builtin_regex(s, {
                                    static REGEX: OnceLock<Regex> = OnceLock::new();
                                    REGEX.get_or_init(|| Regex::new("^(?x)([(<:}])").unwrap())
                                })
                            })
                        })
                    })
                })
                .and_then(|s| parse_modifier_call(s).and_then(|s| s.tag_node("modifier_call")))
        })
    })
}
#[inline]
fn parse_class_domain(state: Input) -> Output {
    state.rule(ValkyrieRule::ClassDomain, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_attribute_call(s).and_then(|s| s.tag_node("attribute_call")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_field_modifier(s).and_then(|s| s.tag_node("field_modifier")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_class_block(s).and_then(|s| s.tag_node("class_block")))
        })
    })
}
#[inline]
fn parse_kw_class(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_CLASS, |s| {
        Err(s)
            .or_else(|s| builtin_text(s, "class", false).and_then(|s| s.tag_node("class")))
            .or_else(|s| builtin_text(s, "structure", false).and_then(|s| s.tag_node("structure")))
    })
}
#[inline]
fn parse_define_union(state: Input) -> Output {
    state.rule(ValkyrieRule::DefineUnion, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_attribute_call(s).and_then(|s| s.tag_node("attribute_call")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_kw_union(s).and_then(|s| s.tag_node("kw_union")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
        })
    })
}
#[inline]
fn parse_kw_union(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_UNION, |s| s.match_string("union", false))
}
#[inline]
fn parse_define_flags(state: Input) -> Output {
    state.rule(ValkyrieRule::DefineFlags, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_attribute_call(s).and_then(|s| s.tag_node("attribute_call")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_kw_flags(s).and_then(|s| s.tag_node("kw_flags")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
        })
    })
}
#[inline]
fn parse_kw_flags(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_FLAGS, |s| s.match_string("flags", false))
}
#[inline]
fn parse_define_trait(state: Input) -> Output {
    state.rule(ValkyrieRule::DefineTrait, |s| parse_kw_trait(s).and_then(|s| s.tag_node("kw_trait")))
}
#[inline]
fn parse_kw_trait(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_TRAIT, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(trait|interface)").unwrap())
        })
    })
}
#[inline]
fn parse_define_function(state: Input) -> Output {
    state.rule(ValkyrieRule::DefineFunction, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_kw_function(s).and_then(|s| s.tag_node("kw_function")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_namepath(s).and_then(|s| s.tag_node("namepath")))
        })
    })
}
#[inline]
fn parse_kw_function(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_FUNCTION, |s| {
        Err(s)
            .or_else(|s| {
                builtin_regex(s, {
                    static REGEX: OnceLock<Regex> = OnceLock::new();
                    REGEX.get_or_init(|| Regex::new("^(?x)(micro|function)").unwrap())
                })
                .and_then(|s| s.tag_node("micro"))
            })
            .or_else(|s| builtin_text(s, "macro", false).and_then(|s| s.tag_node("macro")))
    })
}
#[inline]
fn parse_while_statement(state: Input) -> Output {
    state.rule(ValkyrieRule::WhileStatement, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_kw_while(s).and_then(|s| s.tag_node("kw_while")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_inline_expression(s).and_then(|s| s.tag_node("inline_expression"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_main_statement(s).and_then(|s| s.tag_node("main_statement")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_eos(s)))
        })
    })
}
#[inline]
fn parse_kw_while(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_WHILE, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(while)").unwrap())
        })
    })
}
#[inline]
fn parse_for_statement(state: Input) -> Output {
    state.rule(ValkyrieRule::ForStatement, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_kw_for(s).and_then(|s| s.tag_node("kw_for")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_kw_in(s).and_then(|s| s.tag_node("kw_in")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_inline_expression(s).and_then(|s| s.tag_node("inline_expression"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_main_statement(s).and_then(|s| s.tag_node("main_statement")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_eos(s)))
        })
    })
}
#[inline]
fn parse_main_statement(state: Input) -> Output {
    state.rule(ValkyrieRule::MainStatement, |s| {
        Err(s)
            .or_else(|s| parse_while_statement(s).and_then(|s| s.tag_node("while_statement")))
            .or_else(|s| parse_for_statement(s).and_then(|s| s.tag_node("for_statement")))
            .or_else(|s| parse_main_expression(s).and_then(|s| s.tag_node("main_expression")))
    })
}
#[inline]
fn parse_main_expression(state: Input) -> Output {
    state.rule(ValkyrieRule::MainExpression, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_main_term(s).and_then(|s| s.tag_node("main_term")))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_main_infix(s).and_then(|s| s.tag_node("main_infix")))
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_main_term(s).and_then(|s| s.tag_node("main_term")))
                        })
                    })
                })
                .and_then(|s| s.optional(|s| parse_eos(s).and_then(|s| s.tag_node("eos"))))
        })
    })
}
#[inline]
fn parse_main_term(state: Input) -> Output {
    state.rule(ValkyrieRule::MainTerm, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| parse_main_prefix(s).and_then(|s| s.tag_node("main_prefix")))
                                .and_then(|s| builtin_ignore(s))
                        })
                    })
                })
                .and_then(|s| parse_main_factor(s).and_then(|s| s.tag_node("main_factor")))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_main_suffix(s).and_then(|s| s.tag_node("main_suffix"))))
        })
    })
}
#[inline]
fn parse_main_factor(state: Input) -> Output {
    state.rule(ValkyrieRule::MainFactor, |s| {
        Err(s)
            .or_else(|s| parse_group_factor(s).and_then(|s| s.tag_node("group_factor")))
            .or_else(|s| parse_atomic(s).and_then(|s| s.tag_node("atomic")))
    })
}
#[inline]
fn parse_group_factor(state: Input) -> Output {
    state.rule(ValkyrieRule::GroupFactor, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "(", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_main_expression(s).and_then(|s| s.tag_node("main_expression")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, ")", false))
        })
    })
}
#[inline]
fn parse_atomic(state: Input) -> Output {
    state.rule(ValkyrieRule::Atomic, |s| {
        Err(s)
            .or_else(|s| parse_procedural_call(s).and_then(|s| s.tag_node("procedural_call")))
            .or_else(|s| parse_tuple_literal(s).and_then(|s| s.tag_node("tuple_literal")))
            .or_else(|s| parse_range_literal(s).and_then(|s| s.tag_node("range_literal")))
            .or_else(|s| parse_namepath(s).and_then(|s| s.tag_node("namepath")))
            .or_else(|s| parse_integer(s).and_then(|s| s.tag_node("integer")))
            .or_else(|s| parse_special(s).and_then(|s| s.tag_node("special")))
    })
}
#[inline]
fn parse_main_infix(state: Input) -> Output {
    state.rule(ValkyrieRule::MainInfix, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| {
                Regex::new(
                    "^(?x)([+\\-*/%]=?
    | [\\^√]
    # start with !, =
    | !==|=!=|===|==|=|[!≢≡]
    # start with `<, >`
    | <<<|<<=|<<|[⋘≪⩽≤<]
    | >>>|>>=|>>|[⋙≫⩾≥>]
    # start with &, |
    | [&|]{1,3}
    | [∧⊼⩟∨⊽⊻]
    # range, contains
    | [.]{1,2}[<=]
    | [∈∊∉∋∍∌]
    | (not\\s+)?in
    | is(\\s+not)?
    # map, apply
    | /@ | [⇴⨵⊕⟴] | @{2,3})",
                )
                .unwrap()
            })
        })
    })
}
#[inline]
fn parse_main_prefix(state: Input) -> Output {
    state.rule(ValkyrieRule::MainPrefix, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| {
                Regex::new(
                    "^(?x)([¬!+]
    | [-]
    | [.]{2,3}
    | [⅟]
    | [√∛∜]
    | [&*])",
                )
                .unwrap()
            })
        })
    })
}
#[inline]
fn parse_main_suffix(state: Input) -> Output {
    state.rule(ValkyrieRule::MainSuffix, |s| {
        Err(s).or_else(|s| parse_inline_suffix(s).and_then(|s| s.tag_node("inline_suffix")))
    })
}
#[inline]
fn parse_inline_expression(state: Input) -> Output {
    state.rule(ValkyrieRule::InlineExpression, |s| {
        s.sequence(|s| {
            Ok(s).and_then(|s| parse_inline_term(s).and_then(|s| s.tag_node("inline_term"))).and_then(|s| {
                s.repeat(0..4294967295, |s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_main_infix(s).and_then(|s| s.tag_node("main_infix")))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_inline_term(s).and_then(|s| s.tag_node("inline_term")))
                    })
                })
            })
        })
    })
}
#[inline]
fn parse_inline_term(state: Input) -> Output {
    state.rule(ValkyrieRule::InlineTerm, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| parse_main_prefix(s).and_then(|s| s.tag_node("main_prefix")))
                                .and_then(|s| builtin_ignore(s))
                        })
                    })
                })
                .and_then(|s| parse_main_factor(s).and_then(|s| s.tag_node("main_factor")))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_inline_suffix(s).and_then(|s| s.tag_node("inline_suffix"))))
        })
    })
}
#[inline]
fn parse_inline_suffix(state: Input) -> Output {
    state.rule(ValkyrieRule::InlineSuffix, |s| {
        Err(s)
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| parse_suffix_operator(s).and_then(|s| s.tag_node("suffix_operator")))
                })
                .and_then(|s| s.tag_node("inline_suffix_0"))
            })
            .or_else(|s| parse_tuple_call(s).and_then(|s| s.tag_node("tuple_call")))
            .or_else(|s| parse_range_call(s).and_then(|s| s.tag_node("range_call")))
    })
}
#[inline]
fn parse_suffix_operator(state: Input) -> Output {
    state.rule(ValkyrieRule::SuffixOperator, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| {
                Regex::new(
                    "^(?x)([!]
    | [%‰‱]
    | [′″‴⁗]
    | [℃℉])",
                )
                .unwrap()
            })
        })
    })
}
#[inline]
fn parse_type_hint(state: Input) -> Output {
    state.rule(ValkyrieRule::TypeHint, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_colon(s).and_then(|s| s.tag_node("colon")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_type_expression(s).and_then(|s| s.tag_node("type_expression")))
        })
    })
}
#[inline]
fn parse_type_expression(state: Input) -> Output {
    state.rule(ValkyrieRule::TypeExpression, |s| {
        s.sequence(|s| {
            Ok(s).and_then(|s| parse_type_term(s).and_then(|s| s.tag_node("type_term"))).and_then(|s| {
                s.repeat(0..4294967295, |s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_type_infix(s).and_then(|s| s.tag_node("type_infix")))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_type_term(s).and_then(|s| s.tag_node("type_term")))
                    })
                })
            })
        })
    })
}
#[inline]
fn parse_type_term(state: Input) -> Output {
    state.rule(ValkyrieRule::TypeTerm, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| parse_type_prefix(s).and_then(|s| s.tag_node("type_prefix")))
                                .and_then(|s| builtin_ignore(s))
                        })
                    })
                })
                .and_then(|s| parse_main_factor(s).and_then(|s| s.tag_node("main_factor")))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_type_suffix(s).and_then(|s| s.tag_node("type_suffix"))))
        })
    })
}
#[inline]
fn parse_type_factor(state: Input) -> Output {
    state.rule(ValkyrieRule::TypeFactor, |s| {
        Err(s)
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_text(s, "(", false))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| parse_type_expression(s).and_then(|s| s.tag_node("type_expression")))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| builtin_text(s, ")", false))
                })
                .and_then(|s| s.tag_node("type_factor_0"))
            })
            .or_else(|s| parse_atomic(s).and_then(|s| s.tag_node("atomic")))
    })
}
#[inline]
fn parse_type_infix(state: Input) -> Output {
    state.rule(ValkyrieRule::TypeInfix, |s| Err(s).or_else(|s| builtin_text(s, "|", false).and_then(|s| s.tag_node("union"))))
}
#[inline]
fn parse_type_prefix(state: Input) -> Output {
    state.rule(ValkyrieRule::TypePrefix, |s| {
        Err(s)
            .or_else(|s| {
                builtin_regex(s, {
                    static REGEX: OnceLock<Regex> = OnceLock::new();
                    REGEX.get_or_init(|| Regex::new("^(?x)([+])").unwrap())
                })
                .and_then(|s| s.tag_node("positive"))
            })
            .or_else(|s| {
                builtin_regex(s, {
                    static REGEX: OnceLock<Regex> = OnceLock::new();
                    REGEX.get_or_init(|| Regex::new("^(?x)([-])").unwrap())
                })
                .and_then(|s| s.tag_node("negative"))
            })
    })
}
#[inline]
fn parse_type_suffix(state: Input) -> Output {
    state.rule(ValkyrieRule::TypeSuffix, |s| Err(s).or_else(|s| builtin_text(s, "?", false).and_then(|s| s.tag_node("option"))))
}
#[inline]
fn parse_tuple_call(state: Input) -> Output {
    state.rule(ValkyrieRule::TupleCall, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| s.optional(|s| parse_white_space(s).and_then(|s| s.tag_node("white_space"))))
                .and_then(|s| s.optional(|s| parse_op_and_then(s).and_then(|s| s.tag_node("op_and_then"))))
                .and_then(|s| s.optional(|s| parse_white_space(s).and_then(|s| s.tag_node("white_space"))))
                .and_then(|s| parse_tuple_literal(s).and_then(|s| s.tag_node("tuple_literal")))
        })
    })
}
#[inline]
fn parse_tuple_literal(state: Input) -> Output {
    state.rule(ValkyrieRule::TupleLiteral, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "(", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| parse_tuple_pair(s).and_then(|s| s.tag_node("tuple_pair")))
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| {
                                    s.repeat(0..4294967295, |s| {
                                        s.sequence(|s| {
                                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                                s.sequence(|s| {
                                                    Ok(s).and_then(|s| parse_comma(s)).and_then(|s| builtin_ignore(s)).and_then(
                                                        |s| parse_tuple_pair(s).and_then(|s| s.tag_node("tuple_pair")),
                                                    )
                                                })
                                            })
                                        })
                                    })
                                })
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| s.optional(|s| parse_comma(s)))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, ")", false))
        })
    })
}
#[inline]
fn parse_tuple_pair(state: Input) -> Output {
    state.rule(ValkyrieRule::TuplePair, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| parse_tuple_key(s).and_then(|s| s.tag_node("tuple_key")))
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_colon(s).and_then(|s| s.tag_node("colon")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_main_expression(s).and_then(|s| s.tag_node("main_expression")))
        })
    })
}
#[inline]
fn parse_tuple_key(state: Input) -> Output {
    state.rule(ValkyrieRule::TupleKey, |s| Err(s).or_else(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier"))))
}
#[inline]
fn parse_range_call(state: Input) -> Output {
    state.rule(ValkyrieRule::RangeCall, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| s.optional(|s| parse_white_space(s).and_then(|s| s.tag_node("white_space"))))
                .and_then(|s| s.optional(|s| parse_op_and_then(s).and_then(|s| s.tag_node("op_and_then"))))
                .and_then(|s| s.optional(|s| parse_white_space(s).and_then(|s| s.tag_node("white_space"))))
                .and_then(|s| parse_range_literal(s).and_then(|s| s.tag_node("range_literal")))
        })
    })
}
#[inline]
fn parse_range_literal(state: Input) -> Output {
    state.rule(ValkyrieRule::RangeLiteral, |s| {
        Err(s)
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_text(s, "[", false))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| {
                            s.optional(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| parse_subscript_axis(s).and_then(|s| s.tag_node("subscript_axis")))
                                        .and_then(|s| {
                                            s.repeat(0..4294967295, |s| {
                                                s.sequence(|s| {
                                                    Ok(s)
                                                        .and_then(|s| builtin_ignore(s))
                                                        .and_then(|s| parse_comma(s))
                                                        .and_then(|s| builtin_ignore(s))
                                                        .and_then(|s| {
                                                            parse_subscript_axis(s).and_then(|s| s.tag_node("subscript_axis"))
                                                        })
                                                })
                                            })
                                        })
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| s.optional(|s| parse_comma(s)))
                                })
                            })
                        })
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| builtin_text(s, "]", false))
                })
            })
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_text(s, "⁅", false))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| {
                            s.optional(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| parse_subscript_axis(s).and_then(|s| s.tag_node("subscript_axis")))
                                        .and_then(|s| {
                                            s.repeat(0..4294967295, |s| {
                                                s.sequence(|s| {
                                                    Ok(s)
                                                        .and_then(|s| builtin_ignore(s))
                                                        .and_then(|s| parse_comma(s))
                                                        .and_then(|s| builtin_ignore(s))
                                                        .and_then(|s| {
                                                            parse_subscript_axis(s).and_then(|s| s.tag_node("subscript_axis"))
                                                        })
                                                })
                                            })
                                        })
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| s.optional(|s| parse_comma(s)))
                                })
                            })
                        })
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| builtin_text(s, "⁆", false))
                })
            })
    })
}
#[inline]
fn parse_subscript_axis(state: Input) -> Output {
    state.rule(ValkyrieRule::SubscriptAxis, |s| {
        Err(s)
            .or_else(|s| parse_subscript_range(s).and_then(|s| s.tag_node("subscript_range")))
            .or_else(|s| parse_subscript_only(s).and_then(|s| s.tag_node("subscript_only")))
    })
}
#[inline]
fn parse_subscript_only(state: Input) -> Output {
    state.rule(ValkyrieRule::SubscriptOnly, |s| parse_main_expression(s).and_then(|s| s.tag_node("index")))
}
#[inline]
fn parse_subscript_range(state: Input) -> Output {
    state.rule(ValkyrieRule::SubscriptRange, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| parse_main_expression(s).and_then(|s| s.tag_node("head")))
                                .and_then(|s| builtin_ignore(s))
                        })
                    })
                })
                .and_then(|s| {
                    Err(s)
                        .or_else(|s| {
                            s.sequence(|s| {
                                Ok(s).and_then(|s| parse_range_omit(s)).and_then(|s| {
                                    s.optional(|s| {
                                        s.sequence(|s| {
                                            Ok(s)
                                                .and_then(|s| builtin_ignore(s))
                                                .and_then(|s| parse_main_expression(s).and_then(|s| s.tag_node("step")))
                                        })
                                    })
                                })
                            })
                        })
                        .or_else(|s| {
                            s.sequence(|s| {
                                Ok(s).and_then(|s| parse_colon(s)).and_then(|s| {
                                    s.optional(|s| {
                                        s.sequence(|s| {
                                            Ok(s)
                                                .and_then(|s| builtin_ignore(s))
                                                .and_then(|s| parse_main_expression(s).and_then(|s| s.tag_node("tail")))
                                                .and_then(|s| {
                                                    s.optional(|s| {
                                                        s.sequence(|s| {
                                                            Ok(s)
                                                                .and_then(|s| builtin_ignore(s))
                                                                .and_then(|s| parse_colon(s))
                                                                .and_then(|s| {
                                                                    s.optional(|s| {
                                                                        s.sequence(|s| {
                                                                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(
                                                                                |s| {
                                                                                    parse_main_expression(s)
                                                                                        .and_then(|s| s.tag_node("step"))
                                                                                },
                                                                            )
                                                                        })
                                                                    })
                                                                })
                                                        })
                                                    })
                                                })
                                        })
                                    })
                                })
                            })
                        })
                })
        })
    })
}
#[inline]
fn parse_range_omit(state: Input) -> Output {
    state.rule(ValkyrieRule::RangeOmit, |s| {
        Err(s).or_else(|s| parse_proportion(s).and_then(|s| s.tag_node("proportion"))).or_else(|s| {
            s.sequence(|s| {
                Ok(s)
                    .and_then(|s| parse_colon(s).and_then(|s| s.tag_node("colon")))
                    .and_then(|s| builtin_ignore(s))
                    .and_then(|s| parse_colon(s).and_then(|s| s.tag_node("colon")))
            })
        })
    })
}
#[inline]
fn parse_attribute_call(state: Input) -> Output {
    state.rule(ValkyrieRule::AttributeCall, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_attribute_path(s).and_then(|s| s.tag_node("attribute_path")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_tuple_literal(s).and_then(|s| s.tag_node("tuple_literal"))))
        })
    })
}
#[inline]
fn parse_procedural_call(state: Input) -> Output {
    state.rule(ValkyrieRule::ProceduralCall, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_procedural_path(s).and_then(|s| s.tag_node("procedural_path")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_tuple_literal(s).and_then(|s| s.tag_node("tuple_literal"))))
        })
    })
}
#[inline]
fn parse_modifier_call(state: Input) -> Output {
    state.rule(ValkyrieRule::ModifierCall, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.lookahead(false, |s| {
                        Err(s).or_else(|s| parse_kw_class(s)).or_else(|s| parse_kw_union(s)).or_else(|s| parse_kw_trait(s))
                    })
                })
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
        })
    })
}
#[inline]
fn parse_attribute_path(state: Input) -> Output {
    state.rule(ValkyrieRule::AttributePath, |s| {
        s.sequence(|s| {
            Ok(s).and_then(|s| builtin_text(s, "#", false)).and_then(|s| parse_namepath(s).and_then(|s| s.tag_node("namepath")))
        })
    })
}
#[inline]
fn parse_procedural_path(state: Input) -> Output {
    state.rule(ValkyrieRule::ProceduralPath, |s| {
        s.sequence(|s| {
            Ok(s).and_then(|s| builtin_text(s, "@", false)).and_then(|s| parse_namepath(s).and_then(|s| s.tag_node("namepath")))
        })
    })
}
#[inline]
fn parse_namepath_free(state: Input) -> Output {
    state.rule(ValkyrieRule::NamepathFree, |s| {
        s.sequence(|s| {
            Ok(s).and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier"))).and_then(|s| {
                s.repeat(0..4294967295, |s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_proportion_2(s))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                    })
                })
            })
        })
    })
}
#[inline]
fn parse_namepath(state: Input) -> Output {
    state.rule(ValkyrieRule::Namepath, |s| {
        s.sequence(|s| {
            Ok(s).and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier"))).and_then(|s| {
                s.repeat(0..4294967295, |s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_proportion(s))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                    })
                })
            })
        })
    })
}
#[inline]
fn parse_identifier(state: Input) -> Output {
    state.rule(ValkyrieRule::Identifier, |s| {
        Err(s)
            .or_else(|s| parse_identifier_bare(s).and_then(|s| s.tag_node("identifier_bare")))
            .or_else(|s| parse_identifier_raw(s).and_then(|s| s.tag_node("identifier_raw")))
    })
}
#[inline]
fn parse_identifier_bare(state: Input) -> Output {
    state.rule(ValkyrieRule::IdentifierBare, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([_\\p{XID_start}]\\p{XID_continue}*)").unwrap())
        })
    })
}
#[inline]
fn parse_identifier_raw(state: Input) -> Output {
    state.rule(ValkyrieRule::IdentifierRaw, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "`", false))
                .and_then(|s| parse_identifier_raw_text(s).and_then(|s| s.tag_node("identifier_raw_text")))
                .and_then(|s| builtin_text(s, "`", false))
        })
    })
}
#[inline]
fn parse_identifier_raw_text(state: Input) -> Output {
    state.rule(ValkyrieRule::IdentifierRawText, |s| {
        s.repeat(1..4294967295, |s| {
            s.sequence(|s| {
                Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                    builtin_regex(s, {
                        static REGEX: OnceLock<Regex> = OnceLock::new();
                        REGEX.get_or_init(|| Regex::new("^(?x)([^`])").unwrap())
                    })
                })
            })
        })
    })
}
#[inline]
fn parse_special(state: Input) -> Output {
    state.rule(ValkyrieRule::Special, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([∅∞]|true|false)").unwrap())
        })
    })
}
#[inline]
fn parse_integer(state: Input) -> Output {
    state.rule(ValkyrieRule::Integer, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(0|[1-9][0-9]*)").unwrap())
        })
    })
}
#[inline]
fn parse_proportion(state: Input) -> Output {
    state.rule(ValkyrieRule::PROPORTION, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(∷|::)").unwrap())
        })
    })
}
#[inline]
fn parse_colon(state: Input) -> Output {
    state.rule(ValkyrieRule::COLON, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([:：])").unwrap())
        })
    })
}
#[inline]
fn parse_comma(state: Input) -> Output {
    state.rule(ValkyrieRule::COMMA, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([,，])").unwrap())
        })
    })
}
#[inline]
fn parse_dot(state: Input) -> Output {
    state.rule(ValkyrieRule::DOT, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([.．])").unwrap())
        })
    })
}
#[inline]
fn parse_offset_l(state: Input) -> Output {
    state.rule(ValkyrieRule::OFFSET_L, |s| s.match_string("⁅", false))
}
#[inline]
fn parse_offset_r(state: Input) -> Output {
    state.rule(ValkyrieRule::OFFSET_R, |s| s.match_string("⁆", false))
}
#[inline]
fn parse_proportion_2(state: Input) -> Output {
    state.rule(ValkyrieRule::PROPORTION2, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([.．∷]|::)").unwrap())
        })
    })
}
#[inline]
fn parse_op_import_all(state: Input) -> Output {
    state.rule(ValkyrieRule::OP_IMPORT_ALL, |s| s.match_string("*", false))
}
#[inline]
fn parse_op_and_then(state: Input) -> Output {
    state.rule(ValkyrieRule::OP_AND_THEN, |s| s.match_string("?", false))
}
#[inline]
fn parse_op_bind(state: Input) -> Output {
    state.rule(ValkyrieRule::OP_BIND, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(≔|:=)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_namespace(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_NAMESPACE, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(namespace)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_import(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_IMPORT, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(using)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_template(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_TEMPLATE, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(template|generic)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_where(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_WHERE, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(where)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_implements(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_IMPLEMENTS, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(implements?)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_extends(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_EXTENDS, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(extends?)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_inherits(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_INHERITS, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(inherits?)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_if(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_IF, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(if)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_else(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_ELSE, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(else)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_for(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_FOR, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(for)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_return(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_RETURN, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(return)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_break(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_BREAK, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(break)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_continue(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_CONTINUE, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(continue)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_not(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_NOT, |s| s.match_string("not", false))
}
#[inline]
fn parse_kw_in(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_IN, |s| s.match_string("in", false))
}
#[inline]
fn parse_kw_is(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_IS, |s| s.match_string("is", false))
}
#[inline]
fn parse_kw_as(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_AS, |s| s.match_string("as", false))
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(ValkyrieRule::WhiteSpace, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(\\p{White_Space}+)").unwrap())
        })
    })
}
#[inline]
fn parse_comment(state: Input) -> Output {
    state.rule(ValkyrieRule::Comment, |s| {
        Err(s)
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s).and_then(|s| builtin_text(s, "//", false)).and_then(|s| {
                        builtin_regex(s, {
                            static REGEX: OnceLock<Regex> = OnceLock::new();
                            REGEX.get_or_init(|| Regex::new("^(?x)([^\\n\\r]*)").unwrap())
                        })
                    })
                })
            })
            .or_else(|s| {
                s.sequence(|s| Ok(s).and_then(|s| builtin_text(s, "/*", false)).and_then(|s| builtin_text(s, "*/", false)))
            })
    })
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| parse_white_space(s).or_else(|s| parse_comment(s)))
}

fn builtin_any(state: Input) -> Output {
    state.rule(ValkyrieRule::HiddenText, |s| s.match_char_if(|_| true))
}

fn builtin_text<'i>(state: Input<'i>, text: &'static str, case: bool) -> Output<'i> {
    state.rule(ValkyrieRule::HiddenText, |s| s.match_string(text, case))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(ValkyrieRule::HiddenText, |s| s.match_regex(regex))
}
