use super::*;

pub(super) fn parse_cst(input: &str, rule: ValkyrieRule) -> OutputResult<ValkyrieRule> {
    state(input, |state| match rule {
        ValkyrieRule::Program => parse_program(state),
        ValkyrieRule::Statement => parse_statement(state),
        ValkyrieRule::EOS => parse_eos(state),
        ValkyrieRule::EOS_FREE => parse_eos_free(state),
        ValkyrieRule::DefineNamespace => parse_define_namespace(state),
        ValkyrieRule::KW_NAMESPACE => parse_kw_namespace(state),
        ValkyrieRule::OP_NAMESPACE => parse_op_namespace(state),
        ValkyrieRule::DefineImport => parse_define_import(state),
        ValkyrieRule::ImportTerm => parse_import_term(state),
        ValkyrieRule::ImportAs => parse_import_as(state),
        ValkyrieRule::ImportAll => parse_import_all(state),
        ValkyrieRule::ImportBlock => parse_import_block(state),
        ValkyrieRule::ImportMacro => parse_import_macro(state),
        ValkyrieRule::ImportMacroItem => parse_import_macro_item(state),
        ValkyrieRule::KW_IMPORT => parse_kw_import(state),
        ValkyrieRule::OP_IMPORT_ALL => parse_op_import_all(state),
        ValkyrieRule::DefineClass => parse_define_class(state),
        ValkyrieRule::ClassBlock => parse_class_block(state),
        ValkyrieRule::ClassBlockItem => parse_class_block_item(state),
        ValkyrieRule::ClassInherit => parse_class_inherit(state),
        ValkyrieRule::ClassInheritItem => parse_class_inherit_item(state),
        ValkyrieRule::ClassField => parse_class_field(state),
        ValkyrieRule::ClassMethod => parse_class_method(state),
        ValkyrieRule::ClassDomain => parse_class_domain(state),
        ValkyrieRule::DefineTemplate => parse_define_template(state),
        ValkyrieRule::TemplateParameters => parse_template_parameters(state),
        ValkyrieRule::TemplateBlock => parse_template_block(state),
        ValkyrieRule::TemplateStatement => parse_template_statement(state),
        ValkyrieRule::TemplateImplements => parse_template_implements(state),
        ValkyrieRule::WhereBlock => parse_where_block(state),
        ValkyrieRule::WhereBound => parse_where_bound(state),
        ValkyrieRule::KW_CLASS => parse_kw_class(state),
        ValkyrieRule::KW_UNION => parse_kw_union(state),
        ValkyrieRule::KW_TRAIT => parse_kw_trait(state),
        ValkyrieRule::NamepathFree => parse_namepath_free(state),
        ValkyrieRule::Namepath => parse_namepath(state),
        ValkyrieRule::Identifier => parse_identifier(state),
        ValkyrieRule::IdentifierBare => parse_identifier_bare(state),
        ValkyrieRule::IdentifierRaw => parse_identifier_raw(state),
        ValkyrieRule::IdentifierRawText => parse_identifier_raw_text(state),
        ValkyrieRule::Boolean => parse_boolean(state),
        ValkyrieRule::Integer => parse_integer(state),
        ValkyrieRule::RangeExact => parse_range_exact(state),
        ValkyrieRule::Range => parse_range(state),
        ValkyrieRule::ModifierCall => parse_modifier_call(state),
        ValkyrieRule::COMMA => parse_comma(state),
        ValkyrieRule::KW_TEMPLATE => parse_kw_template(state),
        ValkyrieRule::KW_WHERE => parse_kw_where(state),
        ValkyrieRule::KW_IMPLEMENTS => parse_kw_implements(state),
        ValkyrieRule::KW_AS => parse_kw_as(state),
        ValkyrieRule::KW_IN => parse_kw_in(state),
        ValkyrieRule::WhiteSpace => parse_white_space(state),
        ValkyrieRule::Comment => parse_comment(state),
        ValkyrieRule::IgnoreText => unreachable!(),
        ValkyrieRule::IgnoreRegex => unreachable!(),
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
    })
}
#[inline]
fn parse_eos(state: Input) -> Output {
    state.rule(ValkyrieRule::EOS, |s| {
        Err(s)
            .or_else(|s| {
                builtin_regex(s, {
                    static REGEX: OnceLock<Regex> = OnceLock::new();
                    REGEX.get_or_init(|| Regex::new("^([;；])").unwrap())
                })
                .and_then(|s| s.tag_node("omit"))
            })
            .or_else(|s| {
                builtin_regex(s, {
                    static REGEX: OnceLock<Regex> = OnceLock::new();
                    REGEX.get_or_init(|| Regex::new("^(⁏|;;)").unwrap())
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
            REGEX.get_or_init(|| Regex::new("^([,，;；⁏])").unwrap())
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
fn parse_kw_namespace(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_NAMESPACE, |s| s.match_string("namespace", false))
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
                        .and_then(|s| {
                            s.sequence(|s| {
                                Ok(s).and_then(|s| builtin_text(s, "{", false)).and_then(|s| {
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
                    })
                })
                .and_then(|s| builtin_ignore(s))
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
                        REGEX.get_or_init(|| Regex::new("^([.∷]|::)").unwrap())
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
                            REGEX.get_or_init(|| Regex::new("^([.∷]|::)").unwrap())
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
                                    REGEX.get_or_init(|| Regex::new("^([.∷]|::)").unwrap())
                                })
                            })
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_import_macro_item(s).and_then(|s| s.tag_node("import_macro_item")))
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_kw_as(s))
                    })
                })
                .and_then(|s| builtin_ignore(s))
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
fn parse_kw_import(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_IMPORT, |s| s.match_string("using", false))
}
#[inline]
fn parse_op_import_all(state: Input) -> Output {
    state.rule(ValkyrieRule::OP_IMPORT_ALL, |s| s.match_string("*", false))
}
#[inline]
fn parse_define_class(state: Input) -> Output {
    state.rule(ValkyrieRule::DefineClass, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| s.optional(|s| parse_define_template(s).and_then(|s| s.tag_node("define_template"))))
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
    state.rule(ValkyrieRule::ClassField, |s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
}
#[inline]
fn parse_class_method(state: Input) -> Output {
    state.rule(ValkyrieRule::ClassMethod, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "(", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, ")", false))
        })
    })
}
#[inline]
fn parse_class_domain(state: Input) -> Output {
    state.rule(ValkyrieRule::ClassDomain, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_class_block(s).and_then(|s| s.tag_node("class_block")))
        })
    })
}
#[inline]
fn parse_define_template(state: Input) -> Output {
    state.rule(ValkyrieRule::DefineTemplate, |s| {
        s.sequence(|s| {
            Ok(s)
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
                                                .and_then(|s| parse_comma(s).and_then(|s| s.tag_node("comma")))
                                                .and_then(|s| builtin_ignore(s))
                                                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                                        })
                                    })
                                })
                            })
                        })
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| s.optional(|s| parse_comma(s).and_then(|s| s.tag_node("comma"))))
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
                                                .and_then(|s| parse_comma(s).and_then(|s| s.tag_node("comma")))
                                                .and_then(|s| builtin_ignore(s))
                                                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                                        })
                                    })
                                })
                            })
                        })
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| s.optional(|s| parse_comma(s).and_then(|s| s.tag_node("comma"))))
                        .and_then(|s| builtin_ignore(s))
                        .and_then(|s| builtin_text(s, ">", false))
                })
            })
            .or_else(|s| builtin_text(s, "⟨", false))
            .or_else(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
            .or_else(|s| {
                s.repeat(0..4294967295, |s| {
                    s.sequence(|s| {
                        Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                            s.sequence(|s| {
                                Ok(s)
                                    .and_then(|s| parse_comma(s).and_then(|s| s.tag_node("comma")))
                                    .and_then(|s| builtin_ignore(s))
                                    .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                            })
                        })
                    })
                })
            })
            .or_else(|s| s.optional(|s| parse_comma(s).and_then(|s| s.tag_node("comma"))))
            .or_else(|s| builtin_text(s, "⟩", false))
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
fn parse_kw_class(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_CLASS, |s| {
        Err(s).or_else(|s| builtin_text(s, "class", false)).or_else(|s| builtin_text(s, "structure", false))
    })
}
#[inline]
fn parse_kw_union(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_UNION, |s| s.match_string("union", false))
}
#[inline]
fn parse_kw_trait(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_TRAIT, |s| s.match_string("trait", false))
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
                            .and_then(|s| {
                                builtin_regex(s, {
                                    static REGEX: OnceLock<Regex> = OnceLock::new();
                                    REGEX.get_or_init(|| Regex::new("^([.∷]|::)").unwrap())
                                })
                            })
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
                            .and_then(|s| {
                                builtin_regex(s, {
                                    static REGEX: OnceLock<Regex> = OnceLock::new();
                                    REGEX.get_or_init(|| Regex::new("^(∷|::)").unwrap())
                                })
                            })
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
            REGEX.get_or_init(|| Regex::new("^([_\\p{XID_start}]\\p{XID_continue}*)").unwrap())
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
                        REGEX.get_or_init(|| Regex::new("^([^`])").unwrap())
                    })
                })
            })
        })
    })
}
#[inline]
fn parse_boolean(state: Input) -> Output {
    state.rule(ValkyrieRule::Boolean, |s| {
        Err(s)
            .or_else(|s| builtin_text(s, "true", false).and_then(|s| s.tag_node("true")))
            .or_else(|s| builtin_text(s, "false", false).and_then(|s| s.tag_node("false")))
    })
}
#[inline]
fn parse_integer(state: Input) -> Output {
    state.rule(ValkyrieRule::Integer, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(0|[1-9][0-9]*)").unwrap())
        })
    })
}
#[inline]
fn parse_range_exact(state: Input) -> Output {
    state.rule(ValkyrieRule::RangeExact, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_integer(s).and_then(|s| s.tag_node("integer")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_range(state: Input) -> Output {
    state.rule(ValkyrieRule::Range, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_integer(s).and_then(|s| s.tag_node("min"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, ",", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_integer(s).and_then(|s| s.tag_node("max"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
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
fn parse_comma(state: Input) -> Output {
    state.rule(ValkyrieRule::COMMA, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^([,，])").unwrap())
        })
    })
}
#[inline]
fn parse_kw_template(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_TEMPLATE, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(template|generic)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_where(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_WHERE, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(where)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_implements(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_IMPLEMENTS, |s| s.match_string("implements", false))
}
#[inline]
fn parse_kw_as(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_AS, |s| s.match_string("as", false))
}
#[inline]
fn parse_kw_in(state: Input) -> Output {
    state.rule(ValkyrieRule::KW_IN, |s| s.match_string("as", false))
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(ValkyrieRule::WhiteSpace, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(\\p{White_Space}+)").unwrap())
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
                            REGEX.get_or_init(|| Regex::new("^([^\\n\\r]*)").unwrap())
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
    state.rule(ValkyrieRule::IgnoreText, |s| s.match_char_if(|_| true))
}

fn builtin_text<'i>(state: Input<'i>, text: &'static str, case: bool) -> Output<'i> {
    state.rule(ValkyrieRule::IgnoreText, |s| s.match_string(text, case))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(ValkyrieRule::IgnoreRegex, |s| s.match_regex(regex))
}
