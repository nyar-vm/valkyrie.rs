#![allow(nonstandard_style)]
// Generated from ValkyrieAntlr.g4 by ANTLR 4.8
use super::valkyrieantlrparser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait ValkyrieAntlrListener<'input>: ParseTreeListener<'input, ValkyrieAntlrParserContextType> {
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#program}.
     * @param ctx the parse tree
     */
    fn enter_program(&mut self, _ctx: &ProgramContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#program}.
     * @param ctx the parse tree
     */
    fn exit_program(&mut self, _ctx: &ProgramContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#top_statement}.
     * @param ctx the parse tree
     */
    fn enter_top_statement(&mut self, _ctx: &Top_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#top_statement}.
     * @param ctx the parse tree
     */
    fn exit_top_statement(&mut self, _ctx: &Top_statementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#function_statements}.
     * @param ctx the parse tree
     */
    fn enter_function_statements(&mut self, _ctx: &Function_statementsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#function_statements}.
     * @param ctx the parse tree
     */
    fn exit_function_statements(&mut self, _ctx: &Function_statementsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#eos}.
     * @param ctx the parse tree
     */
    fn enter_eos(&mut self, _ctx: &EosContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#eos}.
     * @param ctx the parse tree
     */
    fn exit_eos(&mut self, _ctx: &EosContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#eos_free}.
     * @param ctx the parse tree
     */
    fn enter_eos_free(&mut self, _ctx: &Eos_freeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#eos_free}.
     * @param ctx the parse tree
     */
    fn exit_eos_free(&mut self, _ctx: &Eos_freeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#define_namespace}.
     * @param ctx the parse tree
     */
    fn enter_define_namespace(&mut self, _ctx: &Define_namespaceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#define_namespace}.
     * @param ctx the parse tree
     */
    fn exit_define_namespace(&mut self, _ctx: &Define_namespaceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#import_statement}.
     * @param ctx the parse tree
     */
    fn enter_import_statement(&mut self, _ctx: &Import_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#import_statement}.
     * @param ctx the parse tree
     */
    fn exit_import_statement(&mut self, _ctx: &Import_statementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#import_as}.
     * @param ctx the parse tree
     */
    fn enter_import_as(&mut self, _ctx: &Import_asContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#import_as}.
     * @param ctx the parse tree
     */
    fn exit_import_as(&mut self, _ctx: &Import_asContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#import_term}.
     * @param ctx the parse tree
     */
    fn enter_import_term(&mut self, _ctx: &Import_termContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#import_term}.
     * @param ctx the parse tree
     */
    fn exit_import_term(&mut self, _ctx: &Import_termContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#import_name}.
     * @param ctx the parse tree
     */
    fn enter_import_name(&mut self, _ctx: &Import_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#import_name}.
     * @param ctx the parse tree
     */
    fn exit_import_name(&mut self, _ctx: &Import_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#import_block}.
     * @param ctx the parse tree
     */
    fn enter_import_block(&mut self, _ctx: &Import_blockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#import_block}.
     * @param ctx the parse tree
     */
    fn exit_import_block(&mut self, _ctx: &Import_blockContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#define_extension}.
     * @param ctx the parse tree
     */
    fn enter_define_extension(&mut self, _ctx: &Define_extensionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#define_extension}.
     * @param ctx the parse tree
     */
    fn exit_define_extension(&mut self, _ctx: &Define_extensionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#define_class}.
     * @param ctx the parse tree
     */
    fn enter_define_class(&mut self, _ctx: &Define_classContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#define_class}.
     * @param ctx the parse tree
     */
    fn exit_define_class(&mut self, _ctx: &Define_classContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#class_block}.
     * @param ctx the parse tree
     */
    fn enter_class_block(&mut self, _ctx: &Class_blockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#class_block}.
     * @param ctx the parse tree
     */
    fn exit_class_block(&mut self, _ctx: &Class_blockContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#class_inherit}.
     * @param ctx the parse tree
     */
    fn enter_class_inherit(&mut self, _ctx: &Class_inheritContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#class_inherit}.
     * @param ctx the parse tree
     */
    fn exit_class_inherit(&mut self, _ctx: &Class_inheritContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#class_inherit_item}.
     * @param ctx the parse tree
     */
    fn enter_class_inherit_item(&mut self, _ctx: &Class_inherit_itemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#class_inherit_item}.
     * @param ctx the parse tree
     */
    fn exit_class_inherit_item(&mut self, _ctx: &Class_inherit_itemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#class_field}.
     * @param ctx the parse tree
     */
    fn enter_class_field(&mut self, _ctx: &Class_fieldContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#class_field}.
     * @param ctx the parse tree
     */
    fn exit_class_field(&mut self, _ctx: &Class_fieldContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#class_method}.
     * @param ctx the parse tree
     */
    fn enter_class_method(&mut self, _ctx: &Class_methodContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#class_method}.
     * @param ctx the parse tree
     */
    fn exit_class_method(&mut self, _ctx: &Class_methodContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#class_dsl}.
     * @param ctx the parse tree
     */
    fn enter_class_dsl(&mut self, _ctx: &Class_dslContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#class_dsl}.
     * @param ctx the parse tree
     */
    fn exit_class_dsl(&mut self, _ctx: &Class_dslContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#define_trait}.
     * @param ctx the parse tree
     */
    fn enter_define_trait(&mut self, _ctx: &Define_traitContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#define_trait}.
     * @param ctx the parse tree
     */
    fn exit_define_trait(&mut self, _ctx: &Define_traitContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#trait_block}.
     * @param ctx the parse tree
     */
    fn enter_trait_block(&mut self, _ctx: &Trait_blockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#trait_block}.
     * @param ctx the parse tree
     */
    fn exit_trait_block(&mut self, _ctx: &Trait_blockContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#define_trait_type}.
     * @param ctx the parse tree
     */
    fn enter_define_trait_type(&mut self, _ctx: &Define_trait_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#define_trait_type}.
     * @param ctx the parse tree
     */
    fn exit_define_trait_type(&mut self, _ctx: &Define_trait_typeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#define_extends}.
     * @param ctx the parse tree
     */
    fn enter_define_extends(&mut self, _ctx: &Define_extendsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#define_extends}.
     * @param ctx the parse tree
     */
    fn exit_define_extends(&mut self, _ctx: &Define_extendsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#impliments}.
     * @param ctx the parse tree
     */
    fn enter_impliments(&mut self, _ctx: &ImplimentsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#impliments}.
     * @param ctx the parse tree
     */
    fn exit_impliments(&mut self, _ctx: &ImplimentsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#define_union}.
     * @param ctx the parse tree
     */
    fn enter_define_union(&mut self, _ctx: &Define_unionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#define_union}.
     * @param ctx the parse tree
     */
    fn exit_define_union(&mut self, _ctx: &Define_unionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#base_layout}.
     * @param ctx the parse tree
     */
    fn enter_base_layout(&mut self, _ctx: &Base_layoutContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#base_layout}.
     * @param ctx the parse tree
     */
    fn exit_base_layout(&mut self, _ctx: &Base_layoutContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#union_block}.
     * @param ctx the parse tree
     */
    fn enter_union_block(&mut self, _ctx: &Union_blockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#union_block}.
     * @param ctx the parse tree
     */
    fn exit_union_block(&mut self, _ctx: &Union_blockContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#define_variant}.
     * @param ctx the parse tree
     */
    fn enter_define_variant(&mut self, _ctx: &Define_variantContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#define_variant}.
     * @param ctx the parse tree
     */
    fn exit_define_variant(&mut self, _ctx: &Define_variantContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#variant_block}.
     * @param ctx the parse tree
     */
    fn enter_variant_block(&mut self, _ctx: &Variant_blockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#variant_block}.
     * @param ctx the parse tree
     */
    fn exit_variant_block(&mut self, _ctx: &Variant_blockContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#define_bitflags}.
     * @param ctx the parse tree
     */
    fn enter_define_bitflags(&mut self, _ctx: &Define_bitflagsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#define_bitflags}.
     * @param ctx the parse tree
     */
    fn exit_define_bitflags(&mut self, _ctx: &Define_bitflagsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#bitflags_block}.
     * @param ctx the parse tree
     */
    fn enter_bitflags_block(&mut self, _ctx: &Bitflags_blockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#bitflags_block}.
     * @param ctx the parse tree
     */
    fn exit_bitflags_block(&mut self, _ctx: &Bitflags_blockContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#bitflags_item}.
     * @param ctx the parse tree
     */
    fn enter_bitflags_item(&mut self, _ctx: &Bitflags_itemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#bitflags_item}.
     * @param ctx the parse tree
     */
    fn exit_bitflags_item(&mut self, _ctx: &Bitflags_itemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#define_function}.
     * @param ctx the parse tree
     */
    fn enter_define_function(&mut self, _ctx: &Define_functionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#define_function}.
     * @param ctx the parse tree
     */
    fn exit_define_function(&mut self, _ctx: &Define_functionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#function_parameters}.
     * @param ctx the parse tree
     */
    fn enter_function_parameters(&mut self, _ctx: &Function_parametersContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#function_parameters}.
     * @param ctx the parse tree
     */
    fn exit_function_parameters(&mut self, _ctx: &Function_parametersContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#parameter_item}.
     * @param ctx the parse tree
     */
    fn enter_parameter_item(&mut self, _ctx: &Parameter_itemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#parameter_item}.
     * @param ctx the parse tree
     */
    fn exit_parameter_item(&mut self, _ctx: &Parameter_itemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#parameter_default}.
     * @param ctx the parse tree
     */
    fn enter_parameter_default(&mut self, _ctx: &Parameter_defaultContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#parameter_default}.
     * @param ctx the parse tree
     */
    fn exit_parameter_default(&mut self, _ctx: &Parameter_defaultContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#function_call}.
     * @param ctx the parse tree
     */
    fn enter_function_call(&mut self, _ctx: &Function_callContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#function_call}.
     * @param ctx the parse tree
     */
    fn exit_function_call(&mut self, _ctx: &Function_callContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#dot_call}.
     * @param ctx the parse tree
     */
    fn enter_dot_call(&mut self, _ctx: &Dot_callContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#dot_call}.
     * @param ctx the parse tree
     */
    fn exit_dot_call(&mut self, _ctx: &Dot_callContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#tuple_call_body}.
     * @param ctx the parse tree
     */
    fn enter_tuple_call_body(&mut self, _ctx: &Tuple_call_bodyContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#tuple_call_body}.
     * @param ctx the parse tree
     */
    fn exit_tuple_call_body(&mut self, _ctx: &Tuple_call_bodyContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#tuple_call_item}.
     * @param ctx the parse tree
     */
    fn enter_tuple_call_item(&mut self, _ctx: &Tuple_call_itemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#tuple_call_item}.
     * @param ctx the parse tree
     */
    fn exit_tuple_call_item(&mut self, _ctx: &Tuple_call_itemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#define_lambda}.
     * @param ctx the parse tree
     */
    fn enter_define_lambda(&mut self, _ctx: &Define_lambdaContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#define_lambda}.
     * @param ctx the parse tree
     */
    fn exit_define_lambda(&mut self, _ctx: &Define_lambdaContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#lambda_call}.
     * @param ctx the parse tree
     */
    fn enter_lambda_call(&mut self, _ctx: &Lambda_callContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#lambda_call}.
     * @param ctx the parse tree
     */
    fn exit_lambda_call(&mut self, _ctx: &Lambda_callContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#function_block}.
     * @param ctx the parse tree
     */
    fn enter_function_block(&mut self, _ctx: &Function_blockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#function_block}.
     * @param ctx the parse tree
     */
    fn exit_function_block(&mut self, _ctx: &Function_blockContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#define_variale}.
     * @param ctx the parse tree
     */
    fn enter_define_variale(&mut self, _ctx: &Define_varialeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#define_variale}.
     * @param ctx the parse tree
     */
    fn exit_define_variale(&mut self, _ctx: &Define_varialeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#let_pattern}.
     * @param ctx the parse tree
     */
    fn enter_let_pattern(&mut self, _ctx: &Let_patternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#let_pattern}.
     * @param ctx the parse tree
     */
    fn exit_let_pattern(&mut self, _ctx: &Let_patternContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#let_pattern_plain}.
     * @param ctx the parse tree
     */
    fn enter_let_pattern_plain(&mut self, _ctx: &Let_pattern_plainContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#let_pattern_plain}.
     * @param ctx the parse tree
     */
    fn exit_let_pattern_plain(&mut self, _ctx: &Let_pattern_plainContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#let_pattern_tuple}.
     * @param ctx the parse tree
     */
    fn enter_let_pattern_tuple(&mut self, _ctx: &Let_pattern_tupleContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#let_pattern_tuple}.
     * @param ctx the parse tree
     */
    fn exit_let_pattern_tuple(&mut self, _ctx: &Let_pattern_tupleContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#let_pattern_item}.
     * @param ctx the parse tree
     */
    fn enter_let_pattern_item(&mut self, _ctx: &Let_pattern_itemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#let_pattern_item}.
     * @param ctx the parse tree
     */
    fn exit_let_pattern_item(&mut self, _ctx: &Let_pattern_itemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#define_type}.
     * @param ctx the parse tree
     */
    fn enter_define_type(&mut self, _ctx: &Define_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#define_type}.
     * @param ctx the parse tree
     */
    fn exit_define_type(&mut self, _ctx: &Define_typeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#type_hint}.
     * @param ctx the parse tree
     */
    fn enter_type_hint(&mut self, _ctx: &Type_hintContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#type_hint}.
     * @param ctx the parse tree
     */
    fn exit_type_hint(&mut self, _ctx: &Type_hintContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#effect_hint}.
     * @param ctx the parse tree
     */
    fn enter_effect_hint(&mut self, _ctx: &Effect_hintContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#effect_hint}.
     * @param ctx the parse tree
     */
    fn exit_effect_hint(&mut self, _ctx: &Effect_hintContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code IfFlow}
     * labeled alternative in {@link ValkyrieAntlrParser#if_statement}.
     * @param ctx the parse tree
     */
    fn enter_IfFlow(&mut self, _ctx: &IfFlowContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code IfFlow}
     * labeled alternative in {@link ValkyrieAntlrParser#if_statement}.
     * @param ctx the parse tree
     */
    fn exit_IfFlow(&mut self, _ctx: &IfFlowContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code IfGuard}
     * labeled alternative in {@link ValkyrieAntlrParser#if_statement}.
     * @param ctx the parse tree
     */
    fn enter_IfGuard(&mut self, _ctx: &IfGuardContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code IfGuard}
     * labeled alternative in {@link ValkyrieAntlrParser#if_statement}.
     * @param ctx the parse tree
     */
    fn exit_IfGuard(&mut self, _ctx: &IfGuardContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#else_if_statement}.
     * @param ctx the parse tree
     */
    fn enter_else_if_statement(&mut self, _ctx: &Else_if_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#else_if_statement}.
     * @param ctx the parse tree
     */
    fn exit_else_if_statement(&mut self, _ctx: &Else_if_statementContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code WhileExpression}
     * labeled alternative in {@link ValkyrieAntlrParser#while_statement}.
     * @param ctx the parse tree
     */
    fn enter_WhileExpression(&mut self, _ctx: &WhileExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code WhileExpression}
     * labeled alternative in {@link ValkyrieAntlrParser#while_statement}.
     * @param ctx the parse tree
     */
    fn exit_WhileExpression(&mut self, _ctx: &WhileExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code WhileLet}
     * labeled alternative in {@link ValkyrieAntlrParser#while_statement}.
     * @param ctx the parse tree
     */
    fn enter_WhileLet(&mut self, _ctx: &WhileLetContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code WhileLet}
     * labeled alternative in {@link ValkyrieAntlrParser#while_statement}.
     * @param ctx the parse tree
     */
    fn exit_WhileLet(&mut self, _ctx: &WhileLetContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#for_statement}.
     * @param ctx the parse tree
     */
    fn enter_for_statement(&mut self, _ctx: &For_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#for_statement}.
     * @param ctx the parse tree
     */
    fn exit_for_statement(&mut self, _ctx: &For_statementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#if_guard}.
     * @param ctx the parse tree
     */
    fn enter_if_guard(&mut self, _ctx: &If_guardContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#if_guard}.
     * @param ctx the parse tree
     */
    fn exit_if_guard(&mut self, _ctx: &If_guardContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#top_expression}.
     * @param ctx the parse tree
     */
    fn enter_top_expression(&mut self, _ctx: &Top_expressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#top_expression}.
     * @param ctx the parse tree
     */
    fn exit_top_expression(&mut self, _ctx: &Top_expressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code EIf}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_EIf(&mut self, _ctx: &EIfContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code EIf}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_EIf(&mut self, _ctx: &EIfContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ESpecial}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_ESpecial(&mut self, _ctx: &ESpecialContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ESpecial}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_ESpecial(&mut self, _ctx: &ESpecialContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code EPlus}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_EPlus(&mut self, _ctx: &EPlusContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code EPlus}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_EPlus(&mut self, _ctx: &EPlusContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ETry}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_ETry(&mut self, _ctx: &ETryContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ETry}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_ETry(&mut self, _ctx: &ETryContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code EObject}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_EObject(&mut self, _ctx: &EObjectContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code EObject}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_EObject(&mut self, _ctx: &EObjectContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ERange}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_ERange(&mut self, _ctx: &ERangeContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ERange}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_ERange(&mut self, _ctx: &ERangeContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code EAssign}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_EAssign(&mut self, _ctx: &EAssignContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code EAssign}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_EAssign(&mut self, _ctx: &EAssignContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ENamepath}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_ENamepath(&mut self, _ctx: &ENamepathContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ENamepath}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_ENamepath(&mut self, _ctx: &ENamepathContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code EMul}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_EMul(&mut self, _ctx: &EMulContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code EMul}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_EMul(&mut self, _ctx: &EMulContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ELambda}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_ELambda(&mut self, _ctx: &ELambdaContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ELambda}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_ELambda(&mut self, _ctx: &ELambdaContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ENumber}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_ENumber(&mut self, _ctx: &ENumberContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ENumber}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_ENumber(&mut self, _ctx: &ENumberContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code EOrElse}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_EOrElse(&mut self, _ctx: &EOrElseContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code EOrElse}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_EOrElse(&mut self, _ctx: &EOrElseContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code EGroup}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_EGroup(&mut self, _ctx: &EGroupContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code EGroup}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_EGroup(&mut self, _ctx: &EGroupContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code EFunction}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_EFunction(&mut self, _ctx: &EFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code EFunction}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_EFunction(&mut self, _ctx: &EFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ESuffix}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_ESuffix(&mut self, _ctx: &ESuffixContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ESuffix}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_ESuffix(&mut self, _ctx: &ESuffixContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ECollection}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_ECollection(&mut self, _ctx: &ECollectionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ECollection}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_ECollection(&mut self, _ctx: &ECollectionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code EMatch}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_EMatch(&mut self, _ctx: &EMatchContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code EMatch}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_EMatch(&mut self, _ctx: &EMatchContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code EMacro}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_EMacro(&mut self, _ctx: &EMacroContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code EMacro}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_EMacro(&mut self, _ctx: &EMacroContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code EPrefix}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_EPrefix(&mut self, _ctx: &EPrefixContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code EPrefix}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_EPrefix(&mut self, _ctx: &EPrefixContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ECompare}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_ECompare(&mut self, _ctx: &ECompareContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ECompare}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_ECompare(&mut self, _ctx: &ECompareContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code EIsA}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_EIsA(&mut self, _ctx: &EIsAContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code EIsA}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_EIsA(&mut self, _ctx: &EIsAContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code EPow}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_EPow(&mut self, _ctx: &EPowContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code EPow}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_EPow(&mut self, _ctx: &EPowContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code EDefine}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_EDefine(&mut self, _ctx: &EDefineContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code EDefine}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_EDefine(&mut self, _ctx: &EDefineContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ELogic}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_ELogic(&mut self, _ctx: &ELogicContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ELogic}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_ELogic(&mut self, _ctx: &ELogicContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code EControl}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_EControl(&mut self, _ctx: &EControlContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code EControl}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_EControl(&mut self, _ctx: &EControlContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ENew}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_ENew(&mut self, _ctx: &ENewContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ENew}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_ENew(&mut self, _ctx: &ENewContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code EString}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_EString(&mut self, _ctx: &EStringContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code EString}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_EString(&mut self, _ctx: &EStringContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ILogic}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_ILogic(&mut self, _ctx: &ILogicContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ILogic}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_ILogic(&mut self, _ctx: &ILogicContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code IDot}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_IDot(&mut self, _ctx: &IDotContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code IDot}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_IDot(&mut self, _ctx: &IDotContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code INamepath}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_INamepath(&mut self, _ctx: &INamepathContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code INamepath}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_INamepath(&mut self, _ctx: &INamepathContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ISpecial}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_ISpecial(&mut self, _ctx: &ISpecialContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ISpecial}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_ISpecial(&mut self, _ctx: &ISpecialContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code IPlus}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_IPlus(&mut self, _ctx: &IPlusContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code IPlus}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_IPlus(&mut self, _ctx: &IPlusContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ICompare}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_ICompare(&mut self, _ctx: &ICompareContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ICompare}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_ICompare(&mut self, _ctx: &ICompareContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code IFunction}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_IFunction(&mut self, _ctx: &IFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code IFunction}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_IFunction(&mut self, _ctx: &IFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code IIsA}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_IIsA(&mut self, _ctx: &IIsAContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code IIsA}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_IIsA(&mut self, _ctx: &IIsAContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code INumber}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_INumber(&mut self, _ctx: &INumberContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code INumber}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_INumber(&mut self, _ctx: &INumberContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code IRange}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_IRange(&mut self, _ctx: &IRangeContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code IRange}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_IRange(&mut self, _ctx: &IRangeContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ICollection}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_ICollection(&mut self, _ctx: &ICollectionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ICollection}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_ICollection(&mut self, _ctx: &ICollectionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code IMul}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_IMul(&mut self, _ctx: &IMulContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code IMul}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_IMul(&mut self, _ctx: &IMulContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code IAs}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_IAs(&mut self, _ctx: &IAsContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code IAs}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_IAs(&mut self, _ctx: &IAsContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code IPrefix}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_IPrefix(&mut self, _ctx: &IPrefixContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code IPrefix}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_IPrefix(&mut self, _ctx: &IPrefixContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code IString}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_IString(&mut self, _ctx: &IStringContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code IString}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_IString(&mut self, _ctx: &IStringContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code IMacro}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_IMacro(&mut self, _ctx: &IMacroContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code IMacro}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_IMacro(&mut self, _ctx: &IMacroContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ISlice}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn enter_ISlice(&mut self, _ctx: &ISliceContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ISlice}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn exit_ISlice(&mut self, _ctx: &ISliceContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code TSpecial}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn enter_TSpecial(&mut self, _ctx: &TSpecialContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code TSpecial}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn exit_TSpecial(&mut self, _ctx: &TSpecialContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code TStrig}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn enter_TStrig(&mut self, _ctx: &TStrigContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code TStrig}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn exit_TStrig(&mut self, _ctx: &TStrigContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code TNumber}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn enter_TNumber(&mut self, _ctx: &TNumberContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code TNumber}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn exit_TNumber(&mut self, _ctx: &TNumberContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code TNamepath}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn enter_TNamepath(&mut self, _ctx: &TNamepathContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code TNamepath}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn exit_TNamepath(&mut self, _ctx: &TNamepathContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code TGeneric}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn enter_TGeneric(&mut self, _ctx: &TGenericContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code TGeneric}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn exit_TGeneric(&mut self, _ctx: &TGenericContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code TPattern}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn enter_TPattern(&mut self, _ctx: &TPatternContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code TPattern}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn exit_TPattern(&mut self, _ctx: &TPatternContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code TTuple}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn enter_TTuple(&mut self, _ctx: &TTupleContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code TTuple}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn exit_TTuple(&mut self, _ctx: &TTupleContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code TAdd}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn enter_TAdd(&mut self, _ctx: &TAddContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code TAdd}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn exit_TAdd(&mut self, _ctx: &TAddContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code TArrows}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn enter_TArrows(&mut self, _ctx: &TArrowsContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code TArrows}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn exit_TArrows(&mut self, _ctx: &TArrowsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#prefix_call}.
     * @param ctx the parse tree
     */
    fn enter_prefix_call(&mut self, _ctx: &Prefix_callContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#prefix_call}.
     * @param ctx the parse tree
     */
    fn exit_prefix_call(&mut self, _ctx: &Prefix_callContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#suffix_call}.
     * @param ctx the parse tree
     */
    fn enter_suffix_call(&mut self, _ctx: &Suffix_callContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#suffix_call}.
     * @param ctx the parse tree
     */
    fn exit_suffix_call(&mut self, _ctx: &Suffix_callContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#control_expression}.
     * @param ctx the parse tree
     */
    fn enter_control_expression(&mut self, _ctx: &Control_expressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#control_expression}.
     * @param ctx the parse tree
     */
    fn exit_control_expression(&mut self, _ctx: &Control_expressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#op_compare}.
     * @param ctx the parse tree
     */
    fn enter_op_compare(&mut self, _ctx: &Op_compareContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#op_compare}.
     * @param ctx the parse tree
     */
    fn exit_op_compare(&mut self, _ctx: &Op_compareContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#op_pattern}.
     * @param ctx the parse tree
     */
    fn enter_op_pattern(&mut self, _ctx: &Op_patternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#op_pattern}.
     * @param ctx the parse tree
     */
    fn exit_op_pattern(&mut self, _ctx: &Op_patternContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#infix_arrows}.
     * @param ctx the parse tree
     */
    fn enter_infix_arrows(&mut self, _ctx: &Infix_arrowsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#infix_arrows}.
     * @param ctx the parse tree
     */
    fn exit_infix_arrows(&mut self, _ctx: &Infix_arrowsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#op_multiple}.
     * @param ctx the parse tree
     */
    fn enter_op_multiple(&mut self, _ctx: &Op_multipleContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#op_multiple}.
     * @param ctx the parse tree
     */
    fn exit_op_multiple(&mut self, _ctx: &Op_multipleContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#op_plus}.
     * @param ctx the parse tree
     */
    fn enter_op_plus(&mut self, _ctx: &Op_plusContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#op_plus}.
     * @param ctx the parse tree
     */
    fn exit_op_plus(&mut self, _ctx: &Op_plusContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#op_logic}.
     * @param ctx the parse tree
     */
    fn enter_op_logic(&mut self, _ctx: &Op_logicContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#op_logic}.
     * @param ctx the parse tree
     */
    fn exit_op_logic(&mut self, _ctx: &Op_logicContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#op_assign}.
     * @param ctx the parse tree
     */
    fn enter_op_assign(&mut self, _ctx: &Op_assignContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#op_assign}.
     * @param ctx the parse tree
     */
    fn exit_op_assign(&mut self, _ctx: &Op_assignContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#infix_is}.
     * @param ctx the parse tree
     */
    fn enter_infix_is(&mut self, _ctx: &Infix_isContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#infix_is}.
     * @param ctx the parse tree
     */
    fn exit_infix_is(&mut self, _ctx: &Infix_isContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#infix_in}.
     * @param ctx the parse tree
     */
    fn enter_infix_in(&mut self, _ctx: &Infix_inContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#infix_in}.
     * @param ctx the parse tree
     */
    fn exit_infix_in(&mut self, _ctx: &Infix_inContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#define_generic}.
     * @param ctx the parse tree
     */
    fn enter_define_generic(&mut self, _ctx: &Define_genericContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#define_generic}.
     * @param ctx the parse tree
     */
    fn exit_define_generic(&mut self, _ctx: &Define_genericContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#generic_item}.
     * @param ctx the parse tree
     */
    fn enter_generic_item(&mut self, _ctx: &Generic_itemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#generic_item}.
     * @param ctx the parse tree
     */
    fn exit_generic_item(&mut self, _ctx: &Generic_itemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#generic_call}.
     * @param ctx the parse tree
     */
    fn enter_generic_call(&mut self, _ctx: &Generic_callContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#generic_call}.
     * @param ctx the parse tree
     */
    fn exit_generic_call(&mut self, _ctx: &Generic_callContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#generic_call_in_type}.
     * @param ctx the parse tree
     */
    fn enter_generic_call_in_type(&mut self, _ctx: &Generic_call_in_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#generic_call_in_type}.
     * @param ctx the parse tree
     */
    fn exit_generic_call_in_type(&mut self, _ctx: &Generic_call_in_typeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#generic_pair}.
     * @param ctx the parse tree
     */
    fn enter_generic_pair(&mut self, _ctx: &Generic_pairContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#generic_pair}.
     * @param ctx the parse tree
     */
    fn exit_generic_pair(&mut self, _ctx: &Generic_pairContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#define_label}.
     * @param ctx the parse tree
     */
    fn enter_define_label(&mut self, _ctx: &Define_labelContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#define_label}.
     * @param ctx the parse tree
     */
    fn exit_define_label(&mut self, _ctx: &Define_labelContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#offset_call}.
     * @param ctx the parse tree
     */
    fn enter_offset_call(&mut self, _ctx: &Offset_callContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#offset_call}.
     * @param ctx the parse tree
     */
    fn exit_offset_call(&mut self, _ctx: &Offset_callContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#template_call}.
     * @param ctx the parse tree
     */
    fn enter_template_call(&mut self, _ctx: &Template_callContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#template_call}.
     * @param ctx the parse tree
     */
    fn exit_template_call(&mut self, _ctx: &Template_callContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#template_block}.
     * @param ctx the parse tree
     */
    fn enter_template_block(&mut self, _ctx: &Template_blockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#template_block}.
     * @param ctx the parse tree
     */
    fn exit_template_block(&mut self, _ctx: &Template_blockContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#tempalte_terms}.
     * @param ctx the parse tree
     */
    fn enter_tempalte_terms(&mut self, _ctx: &Tempalte_termsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#tempalte_terms}.
     * @param ctx the parse tree
     */
    fn exit_tempalte_terms(&mut self, _ctx: &Tempalte_termsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#where_block}.
     * @param ctx the parse tree
     */
    fn enter_where_block(&mut self, _ctx: &Where_blockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#where_block}.
     * @param ctx the parse tree
     */
    fn exit_where_block(&mut self, _ctx: &Where_blockContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#where_bound}.
     * @param ctx the parse tree
     */
    fn enter_where_bound(&mut self, _ctx: &Where_boundContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#where_bound}.
     * @param ctx the parse tree
     */
    fn exit_where_bound(&mut self, _ctx: &Where_boundContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#macro_call}.
     * @param ctx the parse tree
     */
    fn enter_macro_call(&mut self, _ctx: &Macro_callContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#macro_call}.
     * @param ctx the parse tree
     */
    fn exit_macro_call(&mut self, _ctx: &Macro_callContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#annotation}.
     * @param ctx the parse tree
     */
    fn enter_annotation(&mut self, _ctx: &AnnotationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#annotation}.
     * @param ctx the parse tree
     */
    fn exit_annotation(&mut self, _ctx: &AnnotationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#annotation_call_item}.
     * @param ctx the parse tree
     */
    fn enter_annotation_call_item(&mut self, _ctx: &Annotation_call_itemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#annotation_call_item}.
     * @param ctx the parse tree
     */
    fn exit_annotation_call_item(&mut self, _ctx: &Annotation_call_itemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#try_statement}.
     * @param ctx the parse tree
     */
    fn enter_try_statement(&mut self, _ctx: &Try_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#try_statement}.
     * @param ctx the parse tree
     */
    fn exit_try_statement(&mut self, _ctx: &Try_statementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#match_statement}.
     * @param ctx the parse tree
     */
    fn enter_match_statement(&mut self, _ctx: &Match_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#match_statement}.
     * @param ctx the parse tree
     */
    fn exit_match_statement(&mut self, _ctx: &Match_statementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#match_call}.
     * @param ctx the parse tree
     */
    fn enter_match_call(&mut self, _ctx: &Match_callContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#match_call}.
     * @param ctx the parse tree
     */
    fn exit_match_call(&mut self, _ctx: &Match_callContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#match_block}.
     * @param ctx the parse tree
     */
    fn enter_match_block(&mut self, _ctx: &Match_blockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#match_block}.
     * @param ctx the parse tree
     */
    fn exit_match_block(&mut self, _ctx: &Match_blockContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code MatchWith}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn enter_MatchWith(&mut self, _ctx: &MatchWithContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code MatchWith}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn exit_MatchWith(&mut self, _ctx: &MatchWithContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code MatchWithMany}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn enter_MatchWithMany(&mut self, _ctx: &MatchWithManyContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code MatchWithMany}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn exit_MatchWithMany(&mut self, _ctx: &MatchWithManyContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code MatchType}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn enter_MatchType(&mut self, _ctx: &MatchTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code MatchType}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn exit_MatchType(&mut self, _ctx: &MatchTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code MatchWhen}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn enter_MatchWhen(&mut self, _ctx: &MatchWhenContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code MatchWhen}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn exit_MatchWhen(&mut self, _ctx: &MatchWhenContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code MatchElse}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn enter_MatchElse(&mut self, _ctx: &MatchElseContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code MatchElse}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn exit_MatchElse(&mut self, _ctx: &MatchElseContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code MatchCase}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn enter_MatchCase(&mut self, _ctx: &MatchCaseContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code MatchCase}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn exit_MatchCase(&mut self, _ctx: &MatchCaseContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code MatchSkip}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn enter_MatchSkip(&mut self, _ctx: &MatchSkipContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code MatchSkip}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn exit_MatchSkip(&mut self, _ctx: &MatchSkipContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#match_case_block}.
     * @param ctx the parse tree
     */
    fn enter_match_case_block(&mut self, _ctx: &Match_case_blockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#match_case_block}.
     * @param ctx the parse tree
     */
    fn exit_match_case_block(&mut self, _ctx: &Match_case_blockContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code CaseOR}
     * labeled alternative in {@link ValkyrieAntlrParser#case_pattern}.
     * @param ctx the parse tree
     */
    fn enter_CaseOR(&mut self, _ctx: &CaseORContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code CaseOR}
     * labeled alternative in {@link ValkyrieAntlrParser#case_pattern}.
     * @param ctx the parse tree
     */
    fn exit_CaseOR(&mut self, _ctx: &CaseORContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code CaseAtom}
     * labeled alternative in {@link ValkyrieAntlrParser#case_pattern}.
     * @param ctx the parse tree
     */
    fn enter_CaseAtom(&mut self, _ctx: &CaseAtomContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code CaseAtom}
     * labeled alternative in {@link ValkyrieAntlrParser#case_pattern}.
     * @param ctx the parse tree
     */
    fn exit_CaseAtom(&mut self, _ctx: &CaseAtomContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code CaseUntil}
     * labeled alternative in {@link ValkyrieAntlrParser#case_pattern}.
     * @param ctx the parse tree
     */
    fn enter_CaseUntil(&mut self, _ctx: &CaseUntilContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code CaseUntil}
     * labeled alternative in {@link ValkyrieAntlrParser#case_pattern}.
     * @param ctx the parse tree
     */
    fn exit_CaseUntil(&mut self, _ctx: &CaseUntilContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#case_pattern_item}.
     * @param ctx the parse tree
     */
    fn enter_case_pattern_item(&mut self, _ctx: &Case_pattern_itemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#case_pattern_item}.
     * @param ctx the parse tree
     */
    fn exit_case_pattern_item(&mut self, _ctx: &Case_pattern_itemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#case_pattern_tuple}.
     * @param ctx the parse tree
     */
    fn enter_case_pattern_tuple(&mut self, _ctx: &Case_pattern_tupleContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#case_pattern_tuple}.
     * @param ctx the parse tree
     */
    fn exit_case_pattern_tuple(&mut self, _ctx: &Case_pattern_tupleContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#object_statement}.
     * @param ctx the parse tree
     */
    fn enter_object_statement(&mut self, _ctx: &Object_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#object_statement}.
     * @param ctx the parse tree
     */
    fn exit_object_statement(&mut self, _ctx: &Object_statementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#new_statement}.
     * @param ctx the parse tree
     */
    fn enter_new_statement(&mut self, _ctx: &New_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#new_statement}.
     * @param ctx the parse tree
     */
    fn exit_new_statement(&mut self, _ctx: &New_statementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#new_body}.
     * @param ctx the parse tree
     */
    fn enter_new_body(&mut self, _ctx: &New_bodyContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#new_body}.
     * @param ctx the parse tree
     */
    fn exit_new_body(&mut self, _ctx: &New_bodyContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#new_block}.
     * @param ctx the parse tree
     */
    fn enter_new_block(&mut self, _ctx: &New_blockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#new_block}.
     * @param ctx the parse tree
     */
    fn exit_new_block(&mut self, _ctx: &New_blockContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#collection_literal}.
     * @param ctx the parse tree
     */
    fn enter_collection_literal(&mut self, _ctx: &Collection_literalContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#collection_literal}.
     * @param ctx the parse tree
     */
    fn exit_collection_literal(&mut self, _ctx: &Collection_literalContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#collection_pair}.
     * @param ctx the parse tree
     */
    fn enter_collection_pair(&mut self, _ctx: &Collection_pairContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#collection_pair}.
     * @param ctx the parse tree
     */
    fn exit_collection_pair(&mut self, _ctx: &Collection_pairContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#slice_call}.
     * @param ctx the parse tree
     */
    fn enter_slice_call(&mut self, _ctx: &Slice_callContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#slice_call}.
     * @param ctx the parse tree
     */
    fn exit_slice_call(&mut self, _ctx: &Slice_callContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code Ordinal}
     * labeled alternative in {@link ValkyrieAntlrParser#range_literal}.
     * @param ctx the parse tree
     */
    fn enter_Ordinal(&mut self, _ctx: &OrdinalContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code Ordinal}
     * labeled alternative in {@link ValkyrieAntlrParser#range_literal}.
     * @param ctx the parse tree
     */
    fn exit_Ordinal(&mut self, _ctx: &OrdinalContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code Offset}
     * labeled alternative in {@link ValkyrieAntlrParser#range_literal}.
     * @param ctx the parse tree
     */
    fn enter_Offset(&mut self, _ctx: &OffsetContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code Offset}
     * labeled alternative in {@link ValkyrieAntlrParser#range_literal}.
     * @param ctx the parse tree
     */
    fn exit_Offset(&mut self, _ctx: &OffsetContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#range_axis}.
     * @param ctx the parse tree
     */
    fn enter_range_axis(&mut self, _ctx: &Range_axisContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#range_axis}.
     * @param ctx the parse tree
     */
    fn exit_range_axis(&mut self, _ctx: &Range_axisContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#range_start}.
     * @param ctx the parse tree
     */
    fn enter_range_start(&mut self, _ctx: &Range_startContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#range_start}.
     * @param ctx the parse tree
     */
    fn exit_range_start(&mut self, _ctx: &Range_startContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#range_end}.
     * @param ctx the parse tree
     */
    fn enter_range_end(&mut self, _ctx: &Range_endContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#range_end}.
     * @param ctx the parse tree
     */
    fn exit_range_end(&mut self, _ctx: &Range_endContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#range_step}.
     * @param ctx the parse tree
     */
    fn enter_range_step(&mut self, _ctx: &Range_stepContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#range_step}.
     * @param ctx the parse tree
     */
    fn exit_range_step(&mut self, _ctx: &Range_stepContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#modifiers}.
     * @param ctx the parse tree
     */
    fn enter_modifiers(&mut self, _ctx: &ModifiersContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#modifiers}.
     * @param ctx the parse tree
     */
    fn exit_modifiers(&mut self, _ctx: &ModifiersContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#modified_identifier}.
     * @param ctx the parse tree
     */
    fn enter_modified_identifier(&mut self, _ctx: &Modified_identifierContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#modified_identifier}.
     * @param ctx the parse tree
     */
    fn exit_modified_identifier(&mut self, _ctx: &Modified_identifierContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#modified_namepath}.
     * @param ctx the parse tree
     */
    fn enter_modified_namepath(&mut self, _ctx: &Modified_namepathContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#modified_namepath}.
     * @param ctx the parse tree
     */
    fn exit_modified_namepath(&mut self, _ctx: &Modified_namepathContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#lambda_name}.
     * @param ctx the parse tree
     */
    fn enter_lambda_name(&mut self, _ctx: &Lambda_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#lambda_name}.
     * @param ctx the parse tree
     */
    fn exit_lambda_name(&mut self, _ctx: &Lambda_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#function_name}.
     * @param ctx the parse tree
     */
    fn enter_function_name(&mut self, _ctx: &Function_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#function_name}.
     * @param ctx the parse tree
     */
    fn exit_function_name(&mut self, _ctx: &Function_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#namepath_free}.
     * @param ctx the parse tree
     */
    fn enter_namepath_free(&mut self, _ctx: &Namepath_freeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#namepath_free}.
     * @param ctx the parse tree
     */
    fn exit_namepath_free(&mut self, _ctx: &Namepath_freeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#namepath}.
     * @param ctx the parse tree
     */
    fn enter_namepath(&mut self, _ctx: &NamepathContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#namepath}.
     * @param ctx the parse tree
     */
    fn exit_namepath(&mut self, _ctx: &NamepathContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#identifier}.
     * @param ctx the parse tree
     */
    fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#identifier}.
     * @param ctx the parse tree
     */
    fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#number}.
     * @param ctx the parse tree
     */
    fn enter_number(&mut self, _ctx: &NumberContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#number}.
     * @param ctx the parse tree
     */
    fn exit_number(&mut self, _ctx: &NumberContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#number_literal}.
     * @param ctx the parse tree
     */
    fn enter_number_literal(&mut self, _ctx: &Number_literalContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#number_literal}.
     * @param ctx the parse tree
     */
    fn exit_number_literal(&mut self, _ctx: &Number_literalContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#string}.
     * @param ctx the parse tree
     */
    fn enter_string(&mut self, _ctx: &StringContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#string}.
     * @param ctx the parse tree
     */
    fn exit_string(&mut self, _ctx: &StringContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link ValkyrieAntlrParser#string_literal}.
     * @param ctx the parse tree
     */
    fn enter_string_literal(&mut self, _ctx: &String_literalContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link ValkyrieAntlrParser#string_literal}.
     * @param ctx the parse tree
     */
    fn exit_string_literal(&mut self, _ctx: &String_literalContext<'input>) {}
}

antlr_rust::coerce_from! { 'input : ValkyrieAntlrListener<'input> }
