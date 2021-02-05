#![allow(nonstandard_style)]
// Generated from ValkyrieAntlr.g4 by ANTLR 4.8
use super::valkyrieantlrparser::*;
use antlr_rust::tree::{ParseTreeVisitor, ParseTreeVisitorCompat};

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link ValkyrieAntlrParser}.
 */
pub trait ValkyrieAntlrVisitor<'input>: ParseTreeVisitor<'input, ValkyrieAntlrParserContextType> {
    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#program}.
     * @param ctx the parse tree
     */
    fn visit_program(&mut self, ctx: &ProgramContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#eos}.
     * @param ctx the parse tree
     */
    fn visit_eos(&mut self, ctx: &EosContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#eos_free}.
     * @param ctx the parse tree
     */
    fn visit_eos_free(&mut self, ctx: &Eos_freeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_namespace}.
     * @param ctx the parse tree
     */
    fn visit_define_namespace(&mut self, ctx: &Define_namespaceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#import_statement}.
     * @param ctx the parse tree
     */
    fn visit_import_statement(&mut self, ctx: &Import_statementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#import_as}.
     * @param ctx the parse tree
     */
    fn visit_import_as(&mut self, ctx: &Import_asContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#import_term}.
     * @param ctx the parse tree
     */
    fn visit_import_term(&mut self, ctx: &Import_termContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#import_name}.
     * @param ctx the parse tree
     */
    fn visit_import_name(&mut self, ctx: &Import_nameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#import_block}.
     * @param ctx the parse tree
     */
    fn visit_import_block(&mut self, ctx: &Import_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_extension}.
     * @param ctx the parse tree
     */
    fn visit_define_extension(&mut self, ctx: &Define_extensionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_class}.
     * @param ctx the parse tree
     */
    fn visit_define_class(&mut self, ctx: &Define_classContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#class_block}.
     * @param ctx the parse tree
     */
    fn visit_class_block(&mut self, ctx: &Class_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#class_inherit}.
     * @param ctx the parse tree
     */
    fn visit_class_inherit(&mut self, ctx: &Class_inheritContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#class_inherit_item}.
     * @param ctx the parse tree
     */
    fn visit_class_inherit_item(&mut self, ctx: &Class_inherit_itemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#class_field}.
     * @param ctx the parse tree
     */
    fn visit_class_field(&mut self, ctx: &Class_fieldContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#class_method}.
     * @param ctx the parse tree
     */
    fn visit_class_method(&mut self, ctx: &Class_methodContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#class_dsl}.
     * @param ctx the parse tree
     */
    fn visit_class_dsl(&mut self, ctx: &Class_dslContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_trait}.
     * @param ctx the parse tree
     */
    fn visit_define_trait(&mut self, ctx: &Define_traitContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#trait_block}.
     * @param ctx the parse tree
     */
    fn visit_trait_block(&mut self, ctx: &Trait_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_trait_type}.
     * @param ctx the parse tree
     */
    fn visit_define_trait_type(&mut self, ctx: &Define_trait_typeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_extends}.
     * @param ctx the parse tree
     */
    fn visit_define_extends(&mut self, ctx: &Define_extendsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#extends_block}.
     * @param ctx the parse tree
     */
    fn visit_extends_block(&mut self, ctx: &Extends_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#with_implements}.
     * @param ctx the parse tree
     */
    fn visit_with_implements(&mut self, ctx: &With_implementsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_union}.
     * @param ctx the parse tree
     */
    fn visit_define_union(&mut self, ctx: &Define_unionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#base_layout}.
     * @param ctx the parse tree
     */
    fn visit_base_layout(&mut self, ctx: &Base_layoutContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#union_block}.
     * @param ctx the parse tree
     */
    fn visit_union_block(&mut self, ctx: &Union_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_variant}.
     * @param ctx the parse tree
     */
    fn visit_define_variant(&mut self, ctx: &Define_variantContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#variant_block}.
     * @param ctx the parse tree
     */
    fn visit_variant_block(&mut self, ctx: &Variant_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_bitflags}.
     * @param ctx the parse tree
     */
    fn visit_define_bitflags(&mut self, ctx: &Define_bitflagsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#bitflags_block}.
     * @param ctx the parse tree
     */
    fn visit_bitflags_block(&mut self, ctx: &Bitflags_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#bitflags_item}.
     * @param ctx the parse tree
     */
    fn visit_bitflags_item(&mut self, ctx: &Bitflags_itemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_function}.
     * @param ctx the parse tree
     */
    fn visit_define_function(&mut self, ctx: &Define_functionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#function_parameters}.
     * @param ctx the parse tree
     */
    fn visit_function_parameters(&mut self, ctx: &Function_parametersContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#parameter_item}.
     * @param ctx the parse tree
     */
    fn visit_parameter_item(&mut self, ctx: &Parameter_itemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code DictParameters}
     * labeled alternative in {@link ValkyrieAntlrParser#parameter_special}.
     * @param ctx the parse tree
     */
    fn visit_DictParameters(&mut self, ctx: &DictParametersContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ListParameters}
     * labeled alternative in {@link ValkyrieAntlrParser#parameter_special}.
     * @param ctx the parse tree
     */
    fn visit_ListParameters(&mut self, ctx: &ListParametersContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ContextParameter}
     * labeled alternative in {@link ValkyrieAntlrParser#parameter_special}.
     * @param ctx the parse tree
     */
    fn visit_ContextParameter(&mut self, ctx: &ContextParameterContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#return_type}.
     * @param ctx the parse tree
     */
    fn visit_return_type(&mut self, ctx: &Return_typeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#parameter_default}.
     * @param ctx the parse tree
     */
    fn visit_parameter_default(&mut self, ctx: &Parameter_defaultContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#function_call}.
     * @param ctx the parse tree
     */
    fn visit_function_call(&mut self, ctx: &Function_callContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NormalClosure}
     * labeled alternative in {@link ValkyrieAntlrParser#closure_call}.
     * @param ctx the parse tree
     */
    fn visit_NormalClosure(&mut self, ctx: &NormalClosureContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code SlotClosure}
     * labeled alternative in {@link ValkyrieAntlrParser#closure_call}.
     * @param ctx the parse tree
     */
    fn visit_SlotClosure(&mut self, ctx: &SlotClosureContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IntegerClosure}
     * labeled alternative in {@link ValkyrieAntlrParser#closure_call}.
     * @param ctx the parse tree
     */
    fn visit_IntegerClosure(&mut self, ctx: &IntegerClosureContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code InternalClosure}
     * labeled alternative in {@link ValkyrieAntlrParser#closure_call}.
     * @param ctx the parse tree
     */
    fn visit_InternalClosure(&mut self, ctx: &InternalClosureContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#tuple_call_body}.
     * @param ctx the parse tree
     */
    fn visit_tuple_call_body(&mut self, ctx: &Tuple_call_bodyContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#tuple_call_item}.
     * @param ctx the parse tree
     */
    fn visit_tuple_call_item(&mut self, ctx: &Tuple_call_itemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_lambda}.
     * @param ctx the parse tree
     */
    fn visit_define_lambda(&mut self, ctx: &Define_lambdaContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#function_block}.
     * @param ctx the parse tree
     */
    fn visit_function_block(&mut self, ctx: &Function_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#let_binding}.
     * @param ctx the parse tree
     */
    fn visit_let_binding(&mut self, ctx: &Let_bindingContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#let_pattern}.
     * @param ctx the parse tree
     */
    fn visit_let_pattern(&mut self, ctx: &Let_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#let_pattern_plain}.
     * @param ctx the parse tree
     */
    fn visit_let_pattern_plain(&mut self, ctx: &Let_pattern_plainContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#let_pattern_tuple}.
     * @param ctx the parse tree
     */
    fn visit_let_pattern_tuple(&mut self, ctx: &Let_pattern_tupleContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#let_pattern_item}.
     * @param ctx the parse tree
     */
    fn visit_let_pattern_item(&mut self, ctx: &Let_pattern_itemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_type}.
     * @param ctx the parse tree
     */
    fn visit_define_type(&mut self, ctx: &Define_typeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#type_hint}.
     * @param ctx the parse tree
     */
    fn visit_type_hint(&mut self, ctx: &Type_hintContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#if_statement}.
     * @param ctx the parse tree
     */
    fn visit_if_statement(&mut self, ctx: &If_statementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#guard_statement}.
     * @param ctx the parse tree
     */
    fn visit_guard_statement(&mut self, ctx: &Guard_statementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#else_if_statement}.
     * @param ctx the parse tree
     */
    fn visit_else_if_statement(&mut self, ctx: &Else_if_statementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code WhileLoop}
     * labeled alternative in {@link ValkyrieAntlrParser#loop_statement}.
     * @param ctx the parse tree
     */
    fn visit_WhileLoop(&mut self, ctx: &WhileLoopContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code WhileLet}
     * labeled alternative in {@link ValkyrieAntlrParser#loop_statement}.
     * @param ctx the parse tree
     */
    fn visit_WhileLet(&mut self, ctx: &WhileLetContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ForLoop}
     * labeled alternative in {@link ValkyrieAntlrParser#loop_statement}.
     * @param ctx the parse tree
     */
    fn visit_ForLoop(&mut self, ctx: &ForLoopContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#if_guard}.
     * @param ctx the parse tree
     */
    fn visit_if_guard(&mut self, ctx: &If_guardContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#expression_root}.
     * @param ctx the parse tree
     */
    fn visit_expression_root(&mut self, ctx: &Expression_rootContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EIf}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EIf(&mut self, ctx: &EIfContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EUntil}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EUntil(&mut self, ctx: &EUntilContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EIn}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EIn(&mut self, ctx: &EInContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EFloor}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EFloor(&mut self, ctx: &EFloorContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EPlus}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EPlus(&mut self, ctx: &EPlusContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ESlice}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ESlice(&mut self, ctx: &ESliceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EDotMatch}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EDotMatch(&mut self, ctx: &EDotMatchContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ETry}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ETry(&mut self, ctx: &ETryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EAs}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EAs(&mut self, ctx: &EAsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EObject}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EObject(&mut self, ctx: &EObjectContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ETuple}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ETuple(&mut self, ctx: &ETupleContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ERange}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ERange(&mut self, ctx: &ERangeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EAssign}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EAssign(&mut self, ctx: &EAssignContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EAtom}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EAtom(&mut self, ctx: &EAtomContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EMul}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EMul(&mut self, ctx: &EMulContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EPipe}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EPipe(&mut self, ctx: &EPipeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ELambda}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ELambda(&mut self, ctx: &ELambdaContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EOrElse}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EOrElse(&mut self, ctx: &EOrElseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EGroup}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EGroup(&mut self, ctx: &EGroupContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EFunction}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EFunction(&mut self, ctx: &EFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ESuffix}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ESuffix(&mut self, ctx: &ESuffixContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EClosure}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EClosure(&mut self, ctx: &EClosureContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EMatch}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EMatch(&mut self, ctx: &EMatchContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EMacro}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EMacro(&mut self, ctx: &EMacroContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EPrefix}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EPrefix(&mut self, ctx: &EPrefixContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ECompare}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ECompare(&mut self, ctx: &ECompareContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EGeneric}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EGeneric(&mut self, ctx: &EGenericContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EIsA}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EIsA(&mut self, ctx: &EIsAContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EPow}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EPow(&mut self, ctx: &EPowContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EDefine}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EDefine(&mut self, ctx: &EDefineContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EMap}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EMap(&mut self, ctx: &EMapContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ELogic}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ELogic(&mut self, ctx: &ELogicContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EControl}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EControl(&mut self, ctx: &EControlContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ENew}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ENew(&mut self, ctx: &ENewContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ECeiling}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ECeiling(&mut self, ctx: &ECeilingContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ILogic}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_ILogic(&mut self, ctx: &ILogicContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IPlus}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IPlus(&mut self, ctx: &IPlusContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ICompare}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_ICompare(&mut self, ctx: &ICompareContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IGeneric}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IGeneric(&mut self, ctx: &IGenericContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IFunction}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IFunction(&mut self, ctx: &IFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IMap}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IMap(&mut self, ctx: &IMapContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IIs}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IIs(&mut self, ctx: &IIsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ITuple}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_ITuple(&mut self, ctx: &ITupleContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IRange}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IRange(&mut self, ctx: &IRangeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IGroup}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IGroup(&mut self, ctx: &IGroupContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IMul}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IMul(&mut self, ctx: &IMulContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IAs}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IAs(&mut self, ctx: &IAsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IPrefix}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IPrefix(&mut self, ctx: &IPrefixContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IFloor}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IFloor(&mut self, ctx: &IFloorContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IAtom}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IAtom(&mut self, ctx: &IAtomContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ICeiling}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_ICeiling(&mut self, ctx: &ICeilingContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ISlice}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_ISlice(&mut self, ctx: &ISliceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TPrefix}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TPrefix(&mut self, ctx: &TPrefixContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TGeneric}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TGeneric(&mut self, ctx: &TGenericContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TPattern}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TPattern(&mut self, ctx: &TPatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TTuple}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TTuple(&mut self, ctx: &TTupleContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TMust}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TMust(&mut self, ctx: &TMustContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TAtom}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TAtom(&mut self, ctx: &TAtomContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TAdd}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TAdd(&mut self, ctx: &TAddContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TArrows}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TArrows(&mut self, ctx: &TArrowsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TOptional}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TOptional(&mut self, ctx: &TOptionalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code AString}
     * labeled alternative in {@link ValkyrieAntlrParser#atomic}.
     * @param ctx the parse tree
     */
    fn visit_AString(&mut self, ctx: &AStringContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ANumber}
     * labeled alternative in {@link ValkyrieAntlrParser#atomic}.
     * @param ctx the parse tree
     */
    fn visit_ANumber(&mut self, ctx: &ANumberContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ALambda}
     * labeled alternative in {@link ValkyrieAntlrParser#atomic}.
     * @param ctx the parse tree
     */
    fn visit_ALambda(&mut self, ctx: &ALambdaContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code AOutput}
     * labeled alternative in {@link ValkyrieAntlrParser#atomic}.
     * @param ctx the parse tree
     */
    fn visit_AOutput(&mut self, ctx: &AOutputContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ANamepath}
     * labeled alternative in {@link ValkyrieAntlrParser#atomic}.
     * @param ctx the parse tree
     */
    fn visit_ANamepath(&mut self, ctx: &ANamepathContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ASpecial}
     * labeled alternative in {@link ValkyrieAntlrParser#atomic}.
     * @param ctx the parse tree
     */
    fn visit_ASpecial(&mut self, ctx: &ASpecialContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CReturn}
     * labeled alternative in {@link ValkyrieAntlrParser#control_expression}.
     * @param ctx the parse tree
     */
    fn visit_CReturn(&mut self, ctx: &CReturnContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CBreak}
     * labeled alternative in {@link ValkyrieAntlrParser#control_expression}.
     * @param ctx the parse tree
     */
    fn visit_CBreak(&mut self, ctx: &CBreakContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CContinue}
     * labeled alternative in {@link ValkyrieAntlrParser#control_expression}.
     * @param ctx the parse tree
     */
    fn visit_CContinue(&mut self, ctx: &CContinueContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CRaise}
     * labeled alternative in {@link ValkyrieAntlrParser#control_expression}.
     * @param ctx the parse tree
     */
    fn visit_CRaise(&mut self, ctx: &CRaiseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CYield}
     * labeled alternative in {@link ValkyrieAntlrParser#control_expression}.
     * @param ctx the parse tree
     */
    fn visit_CYield(&mut self, ctx: &CYieldContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CWith}
     * labeled alternative in {@link ValkyrieAntlrParser#control_expression}.
     * @param ctx the parse tree
     */
    fn visit_CWith(&mut self, ctx: &CWithContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CFall}
     * labeled alternative in {@link ValkyrieAntlrParser#control_expression}.
     * @param ctx the parse tree
     */
    fn visit_CFall(&mut self, ctx: &CFallContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#op_prefix}.
     * @param ctx the parse tree
     */
    fn visit_op_prefix(&mut self, ctx: &Op_prefixContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#op_suffix}.
     * @param ctx the parse tree
     */
    fn visit_op_suffix(&mut self, ctx: &Op_suffixContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#op_compare}.
     * @param ctx the parse tree
     */
    fn visit_op_compare(&mut self, ctx: &Op_compareContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#op_pattern}.
     * @param ctx the parse tree
     */
    fn visit_op_pattern(&mut self, ctx: &Op_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#infix_map}.
     * @param ctx the parse tree
     */
    fn visit_infix_map(&mut self, ctx: &Infix_mapContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#infix_pow}.
     * @param ctx the parse tree
     */
    fn visit_infix_pow(&mut self, ctx: &Infix_powContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#infix_range}.
     * @param ctx the parse tree
     */
    fn visit_infix_range(&mut self, ctx: &Infix_rangeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#infix_arrows}.
     * @param ctx the parse tree
     */
    fn visit_infix_arrows(&mut self, ctx: &Infix_arrowsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#op_multiple}.
     * @param ctx the parse tree
     */
    fn visit_op_multiple(&mut self, ctx: &Op_multipleContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#op_plus}.
     * @param ctx the parse tree
     */
    fn visit_op_plus(&mut self, ctx: &Op_plusContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#op_logic}.
     * @param ctx the parse tree
     */
    fn visit_op_logic(&mut self, ctx: &Op_logicContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#op_pipeline}.
     * @param ctx the parse tree
     */
    fn visit_op_pipeline(&mut self, ctx: &Op_pipelineContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#infix_assign}.
     * @param ctx the parse tree
     */
    fn visit_infix_assign(&mut self, ctx: &Infix_assignContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#infix_is}.
     * @param ctx the parse tree
     */
    fn visit_infix_is(&mut self, ctx: &Infix_isContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#infix_as}.
     * @param ctx the parse tree
     */
    fn visit_infix_as(&mut self, ctx: &Infix_asContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#infix_in}.
     * @param ctx the parse tree
     */
    fn visit_infix_in(&mut self, ctx: &Infix_inContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_generic}.
     * @param ctx the parse tree
     */
    fn visit_define_generic(&mut self, ctx: &Define_genericContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#generic_item}.
     * @param ctx the parse tree
     */
    fn visit_generic_item(&mut self, ctx: &Generic_itemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#generic_call}.
     * @param ctx the parse tree
     */
    fn visit_generic_call(&mut self, ctx: &Generic_callContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#generic_call_in_type}.
     * @param ctx the parse tree
     */
    fn visit_generic_call_in_type(&mut self, ctx: &Generic_call_in_typeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#generic_pair}.
     * @param ctx the parse tree
     */
    fn visit_generic_pair(&mut self, ctx: &Generic_pairContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_label}.
     * @param ctx the parse tree
     */
    fn visit_define_label(&mut self, ctx: &Define_labelContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#template_call}.
     * @param ctx the parse tree
     */
    fn visit_template_call(&mut self, ctx: &Template_callContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#template_block}.
     * @param ctx the parse tree
     */
    fn visit_template_block(&mut self, ctx: &Template_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#template_statements}.
     * @param ctx the parse tree
     */
    fn visit_template_statements(&mut self, ctx: &Template_statementsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#template_implements}.
     * @param ctx the parse tree
     */
    fn visit_template_implements(&mut self, ctx: &Template_implementsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#where_block}.
     * @param ctx the parse tree
     */
    fn visit_where_block(&mut self, ctx: &Where_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#where_bound}.
     * @param ctx the parse tree
     */
    fn visit_where_bound(&mut self, ctx: &Where_boundContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#require_block}.
     * @param ctx the parse tree
     */
    fn visit_require_block(&mut self, ctx: &Require_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#macro_call}.
     * @param ctx the parse tree
     */
    fn visit_macro_call(&mut self, ctx: &Macro_callContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#annotation}.
     * @param ctx the parse tree
     */
    fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#annotation_call_item}.
     * @param ctx the parse tree
     */
    fn visit_annotation_call_item(&mut self, ctx: &Annotation_call_itemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#try_statement}.
     * @param ctx the parse tree
     */
    fn visit_try_statement(&mut self, ctx: &Try_statementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#match_statement}.
     * @param ctx the parse tree
     */
    fn visit_match_statement(&mut self, ctx: &Match_statementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#match_block}.
     * @param ctx the parse tree
     */
    fn visit_match_block(&mut self, ctx: &Match_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code MatchWith}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn visit_MatchWith(&mut self, ctx: &MatchWithContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code MatchWithMany}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn visit_MatchWithMany(&mut self, ctx: &MatchWithManyContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code MatchType}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn visit_MatchType(&mut self, ctx: &MatchTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code MatchWhen}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn visit_MatchWhen(&mut self, ctx: &MatchWhenContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code MatchElse}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn visit_MatchElse(&mut self, ctx: &MatchElseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code MatchCase}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn visit_MatchCase(&mut self, ctx: &MatchCaseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#match_case_block}.
     * @param ctx the parse tree
     */
    fn visit_match_case_block(&mut self, ctx: &Match_case_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CaseOR}
     * labeled alternative in {@link ValkyrieAntlrParser#case_pattern}.
     * @param ctx the parse tree
     */
    fn visit_CaseOR(&mut self, ctx: &CaseORContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CaseAtom}
     * labeled alternative in {@link ValkyrieAntlrParser#case_pattern}.
     * @param ctx the parse tree
     */
    fn visit_CaseAtom(&mut self, ctx: &CaseAtomContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CaseUntil}
     * labeled alternative in {@link ValkyrieAntlrParser#case_pattern}.
     * @param ctx the parse tree
     */
    fn visit_CaseUntil(&mut self, ctx: &CaseUntilContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#case_pattern_item}.
     * @param ctx the parse tree
     */
    fn visit_case_pattern_item(&mut self, ctx: &Case_pattern_itemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#case_pattern_tuple}.
     * @param ctx the parse tree
     */
    fn visit_case_pattern_tuple(&mut self, ctx: &Case_pattern_tupleContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#object_statement}.
     * @param ctx the parse tree
     */
    fn visit_object_statement(&mut self, ctx: &Object_statementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#new_statement}.
     * @param ctx the parse tree
     */
    fn visit_new_statement(&mut self, ctx: &New_statementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#new_block}.
     * @param ctx the parse tree
     */
    fn visit_new_block(&mut self, ctx: &New_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#new_call_item}.
     * @param ctx the parse tree
     */
    fn visit_new_call_item(&mut self, ctx: &New_call_itemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NK1}
     * labeled alternative in {@link ValkyrieAntlrParser#new_call_key}.
     * @param ctx the parse tree
     */
    fn visit_NK1(&mut self, ctx: &NK1Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NK2}
     * labeled alternative in {@link ValkyrieAntlrParser#new_call_key}.
     * @param ctx the parse tree
     */
    fn visit_NK2(&mut self, ctx: &NK2Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NK3}
     * labeled alternative in {@link ValkyrieAntlrParser#new_call_key}.
     * @param ctx the parse tree
     */
    fn visit_NK3(&mut self, ctx: &NK3Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NK4}
     * labeled alternative in {@link ValkyrieAntlrParser#new_call_key}.
     * @param ctx the parse tree
     */
    fn visit_NK4(&mut self, ctx: &NK4Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#tuple_literal}.
     * @param ctx the parse tree
     */
    fn visit_tuple_literal(&mut self, ctx: &Tuple_literalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#collection_pair}.
     * @param ctx the parse tree
     */
    fn visit_collection_pair(&mut self, ctx: &Collection_pairContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CK1}
     * labeled alternative in {@link ValkyrieAntlrParser#collection_key}.
     * @param ctx the parse tree
     */
    fn visit_CK1(&mut self, ctx: &CK1Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CK2}
     * labeled alternative in {@link ValkyrieAntlrParser#collection_key}.
     * @param ctx the parse tree
     */
    fn visit_CK2(&mut self, ctx: &CK2Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CK3}
     * labeled alternative in {@link ValkyrieAntlrParser#collection_key}.
     * @param ctx the parse tree
     */
    fn visit_CK3(&mut self, ctx: &CK3Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#slice_call}.
     * @param ctx the parse tree
     */
    fn visit_slice_call(&mut self, ctx: &Slice_callContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#range_literal}.
     * @param ctx the parse tree
     */
    fn visit_range_literal(&mut self, ctx: &Range_literalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#range_axis}.
     * @param ctx the parse tree
     */
    fn visit_range_axis(&mut self, ctx: &Range_axisContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#modifiers}.
     * @param ctx the parse tree
     */
    fn visit_modifiers(&mut self, ctx: &ModifiersContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#modified_identifier}.
     * @param ctx the parse tree
     */
    fn visit_modified_identifier(&mut self, ctx: &Modified_identifierContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#modified_namepath}.
     * @param ctx the parse tree
     */
    fn visit_modified_namepath(&mut self, ctx: &Modified_namepathContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#lambda_name}.
     * @param ctx the parse tree
     */
    fn visit_lambda_name(&mut self, ctx: &Lambda_nameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code PositiveOutput}
     * labeled alternative in {@link ValkyrieAntlrParser#output_name}.
     * @param ctx the parse tree
     */
    fn visit_PositiveOutput(&mut self, ctx: &PositiveOutputContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NegativeOutput}
     * labeled alternative in {@link ValkyrieAntlrParser#output_name}.
     * @param ctx the parse tree
     */
    fn visit_NegativeOutput(&mut self, ctx: &NegativeOutputContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#namepath_free}.
     * @param ctx the parse tree
     */
    fn visit_namepath_free(&mut self, ctx: &Namepath_freeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#namepath}.
     * @param ctx the parse tree
     */
    fn visit_namepath(&mut self, ctx: &NamepathContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#identifier}.
     * @param ctx the parse tree
     */
    fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#number}.
     * @param ctx the parse tree
     */
    fn visit_number(&mut self, ctx: &NumberContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#number_literal}.
     * @param ctx the parse tree
     */
    fn visit_number_literal(&mut self, ctx: &Number_literalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#string}.
     * @param ctx the parse tree
     */
    fn visit_string(&mut self, ctx: &StringContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#string_literal}.
     * @param ctx the parse tree
     */
    fn visit_string_literal(&mut self, ctx: &String_literalContext<'input>) {
        self.visit_children(ctx)
    }
}

pub trait ValkyrieAntlrVisitorCompat<'input>: ParseTreeVisitorCompat<'input, Node = ValkyrieAntlrParserContextType> {
    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#program}.
     * @param ctx the parse tree
     */
    fn visit_program(&mut self, ctx: &ProgramContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#eos}.
     * @param ctx the parse tree
     */
    fn visit_eos(&mut self, ctx: &EosContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#eos_free}.
     * @param ctx the parse tree
     */
    fn visit_eos_free(&mut self, ctx: &Eos_freeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_namespace}.
     * @param ctx the parse tree
     */
    fn visit_define_namespace(&mut self, ctx: &Define_namespaceContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#import_statement}.
     * @param ctx the parse tree
     */
    fn visit_import_statement(&mut self, ctx: &Import_statementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#import_as}.
     * @param ctx the parse tree
     */
    fn visit_import_as(&mut self, ctx: &Import_asContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#import_term}.
     * @param ctx the parse tree
     */
    fn visit_import_term(&mut self, ctx: &Import_termContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#import_name}.
     * @param ctx the parse tree
     */
    fn visit_import_name(&mut self, ctx: &Import_nameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#import_block}.
     * @param ctx the parse tree
     */
    fn visit_import_block(&mut self, ctx: &Import_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_extension}.
     * @param ctx the parse tree
     */
    fn visit_define_extension(&mut self, ctx: &Define_extensionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_class}.
     * @param ctx the parse tree
     */
    fn visit_define_class(&mut self, ctx: &Define_classContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#class_block}.
     * @param ctx the parse tree
     */
    fn visit_class_block(&mut self, ctx: &Class_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#class_inherit}.
     * @param ctx the parse tree
     */
    fn visit_class_inherit(&mut self, ctx: &Class_inheritContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#class_inherit_item}.
     * @param ctx the parse tree
     */
    fn visit_class_inherit_item(&mut self, ctx: &Class_inherit_itemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#class_field}.
     * @param ctx the parse tree
     */
    fn visit_class_field(&mut self, ctx: &Class_fieldContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#class_method}.
     * @param ctx the parse tree
     */
    fn visit_class_method(&mut self, ctx: &Class_methodContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#class_dsl}.
     * @param ctx the parse tree
     */
    fn visit_class_dsl(&mut self, ctx: &Class_dslContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_trait}.
     * @param ctx the parse tree
     */
    fn visit_define_trait(&mut self, ctx: &Define_traitContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#trait_block}.
     * @param ctx the parse tree
     */
    fn visit_trait_block(&mut self, ctx: &Trait_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_trait_type}.
     * @param ctx the parse tree
     */
    fn visit_define_trait_type(&mut self, ctx: &Define_trait_typeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_extends}.
     * @param ctx the parse tree
     */
    fn visit_define_extends(&mut self, ctx: &Define_extendsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#extends_block}.
     * @param ctx the parse tree
     */
    fn visit_extends_block(&mut self, ctx: &Extends_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#with_implements}.
     * @param ctx the parse tree
     */
    fn visit_with_implements(&mut self, ctx: &With_implementsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_union}.
     * @param ctx the parse tree
     */
    fn visit_define_union(&mut self, ctx: &Define_unionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#base_layout}.
     * @param ctx the parse tree
     */
    fn visit_base_layout(&mut self, ctx: &Base_layoutContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#union_block}.
     * @param ctx the parse tree
     */
    fn visit_union_block(&mut self, ctx: &Union_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_variant}.
     * @param ctx the parse tree
     */
    fn visit_define_variant(&mut self, ctx: &Define_variantContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#variant_block}.
     * @param ctx the parse tree
     */
    fn visit_variant_block(&mut self, ctx: &Variant_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_bitflags}.
     * @param ctx the parse tree
     */
    fn visit_define_bitflags(&mut self, ctx: &Define_bitflagsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#bitflags_block}.
     * @param ctx the parse tree
     */
    fn visit_bitflags_block(&mut self, ctx: &Bitflags_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#bitflags_item}.
     * @param ctx the parse tree
     */
    fn visit_bitflags_item(&mut self, ctx: &Bitflags_itemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_function}.
     * @param ctx the parse tree
     */
    fn visit_define_function(&mut self, ctx: &Define_functionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#function_parameters}.
     * @param ctx the parse tree
     */
    fn visit_function_parameters(&mut self, ctx: &Function_parametersContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#parameter_item}.
     * @param ctx the parse tree
     */
    fn visit_parameter_item(&mut self, ctx: &Parameter_itemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code DictParameters}
     * labeled alternative in {@link ValkyrieAntlrParser#parameter_special}.
     * @param ctx the parse tree
     */
    fn visit_DictParameters(&mut self, ctx: &DictParametersContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ListParameters}
     * labeled alternative in {@link ValkyrieAntlrParser#parameter_special}.
     * @param ctx the parse tree
     */
    fn visit_ListParameters(&mut self, ctx: &ListParametersContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ContextParameter}
     * labeled alternative in {@link ValkyrieAntlrParser#parameter_special}.
     * @param ctx the parse tree
     */
    fn visit_ContextParameter(&mut self, ctx: &ContextParameterContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#return_type}.
     * @param ctx the parse tree
     */
    fn visit_return_type(&mut self, ctx: &Return_typeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#parameter_default}.
     * @param ctx the parse tree
     */
    fn visit_parameter_default(&mut self, ctx: &Parameter_defaultContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#function_call}.
     * @param ctx the parse tree
     */
    fn visit_function_call(&mut self, ctx: &Function_callContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NormalClosure}
     * labeled alternative in {@link ValkyrieAntlrParser#closure_call}.
     * @param ctx the parse tree
     */
    fn visit_NormalClosure(&mut self, ctx: &NormalClosureContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code SlotClosure}
     * labeled alternative in {@link ValkyrieAntlrParser#closure_call}.
     * @param ctx the parse tree
     */
    fn visit_SlotClosure(&mut self, ctx: &SlotClosureContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IntegerClosure}
     * labeled alternative in {@link ValkyrieAntlrParser#closure_call}.
     * @param ctx the parse tree
     */
    fn visit_IntegerClosure(&mut self, ctx: &IntegerClosureContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code InternalClosure}
     * labeled alternative in {@link ValkyrieAntlrParser#closure_call}.
     * @param ctx the parse tree
     */
    fn visit_InternalClosure(&mut self, ctx: &InternalClosureContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#tuple_call_body}.
     * @param ctx the parse tree
     */
    fn visit_tuple_call_body(&mut self, ctx: &Tuple_call_bodyContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#tuple_call_item}.
     * @param ctx the parse tree
     */
    fn visit_tuple_call_item(&mut self, ctx: &Tuple_call_itemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_lambda}.
     * @param ctx the parse tree
     */
    fn visit_define_lambda(&mut self, ctx: &Define_lambdaContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#function_block}.
     * @param ctx the parse tree
     */
    fn visit_function_block(&mut self, ctx: &Function_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#let_binding}.
     * @param ctx the parse tree
     */
    fn visit_let_binding(&mut self, ctx: &Let_bindingContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#let_pattern}.
     * @param ctx the parse tree
     */
    fn visit_let_pattern(&mut self, ctx: &Let_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#let_pattern_plain}.
     * @param ctx the parse tree
     */
    fn visit_let_pattern_plain(&mut self, ctx: &Let_pattern_plainContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#let_pattern_tuple}.
     * @param ctx the parse tree
     */
    fn visit_let_pattern_tuple(&mut self, ctx: &Let_pattern_tupleContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#let_pattern_item}.
     * @param ctx the parse tree
     */
    fn visit_let_pattern_item(&mut self, ctx: &Let_pattern_itemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_type}.
     * @param ctx the parse tree
     */
    fn visit_define_type(&mut self, ctx: &Define_typeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#type_hint}.
     * @param ctx the parse tree
     */
    fn visit_type_hint(&mut self, ctx: &Type_hintContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#if_statement}.
     * @param ctx the parse tree
     */
    fn visit_if_statement(&mut self, ctx: &If_statementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#guard_statement}.
     * @param ctx the parse tree
     */
    fn visit_guard_statement(&mut self, ctx: &Guard_statementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#else_if_statement}.
     * @param ctx the parse tree
     */
    fn visit_else_if_statement(&mut self, ctx: &Else_if_statementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code WhileLoop}
     * labeled alternative in {@link ValkyrieAntlrParser#loop_statement}.
     * @param ctx the parse tree
     */
    fn visit_WhileLoop(&mut self, ctx: &WhileLoopContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code WhileLet}
     * labeled alternative in {@link ValkyrieAntlrParser#loop_statement}.
     * @param ctx the parse tree
     */
    fn visit_WhileLet(&mut self, ctx: &WhileLetContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ForLoop}
     * labeled alternative in {@link ValkyrieAntlrParser#loop_statement}.
     * @param ctx the parse tree
     */
    fn visit_ForLoop(&mut self, ctx: &ForLoopContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#if_guard}.
     * @param ctx the parse tree
     */
    fn visit_if_guard(&mut self, ctx: &If_guardContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#expression_root}.
     * @param ctx the parse tree
     */
    fn visit_expression_root(&mut self, ctx: &Expression_rootContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EIf}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EIf(&mut self, ctx: &EIfContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EUntil}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EUntil(&mut self, ctx: &EUntilContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EIn}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EIn(&mut self, ctx: &EInContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EFloor}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EFloor(&mut self, ctx: &EFloorContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EPlus}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EPlus(&mut self, ctx: &EPlusContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ESlice}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ESlice(&mut self, ctx: &ESliceContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EDotMatch}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EDotMatch(&mut self, ctx: &EDotMatchContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ETry}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ETry(&mut self, ctx: &ETryContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EAs}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EAs(&mut self, ctx: &EAsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EObject}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EObject(&mut self, ctx: &EObjectContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ETuple}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ETuple(&mut self, ctx: &ETupleContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ERange}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ERange(&mut self, ctx: &ERangeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EAssign}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EAssign(&mut self, ctx: &EAssignContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EAtom}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EAtom(&mut self, ctx: &EAtomContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EMul}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EMul(&mut self, ctx: &EMulContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EPipe}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EPipe(&mut self, ctx: &EPipeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ELambda}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ELambda(&mut self, ctx: &ELambdaContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EOrElse}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EOrElse(&mut self, ctx: &EOrElseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EGroup}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EGroup(&mut self, ctx: &EGroupContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EFunction}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EFunction(&mut self, ctx: &EFunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ESuffix}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ESuffix(&mut self, ctx: &ESuffixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EClosure}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EClosure(&mut self, ctx: &EClosureContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EMatch}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EMatch(&mut self, ctx: &EMatchContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EMacro}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EMacro(&mut self, ctx: &EMacroContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EPrefix}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EPrefix(&mut self, ctx: &EPrefixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ECompare}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ECompare(&mut self, ctx: &ECompareContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EGeneric}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EGeneric(&mut self, ctx: &EGenericContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EIsA}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EIsA(&mut self, ctx: &EIsAContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EPow}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EPow(&mut self, ctx: &EPowContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EDefine}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EDefine(&mut self, ctx: &EDefineContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EMap}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EMap(&mut self, ctx: &EMapContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ELogic}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ELogic(&mut self, ctx: &ELogicContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code EControl}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_EControl(&mut self, ctx: &EControlContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ENew}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ENew(&mut self, ctx: &ENewContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ECeiling}
     * labeled alternative in {@link ValkyrieAntlrParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_ECeiling(&mut self, ctx: &ECeilingContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ILogic}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_ILogic(&mut self, ctx: &ILogicContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IPlus}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IPlus(&mut self, ctx: &IPlusContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ICompare}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_ICompare(&mut self, ctx: &ICompareContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IGeneric}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IGeneric(&mut self, ctx: &IGenericContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IFunction}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IFunction(&mut self, ctx: &IFunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IMap}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IMap(&mut self, ctx: &IMapContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IIs}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IIs(&mut self, ctx: &IIsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ITuple}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_ITuple(&mut self, ctx: &ITupleContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IRange}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IRange(&mut self, ctx: &IRangeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IGroup}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IGroup(&mut self, ctx: &IGroupContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IMul}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IMul(&mut self, ctx: &IMulContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IAs}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IAs(&mut self, ctx: &IAsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IPrefix}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IPrefix(&mut self, ctx: &IPrefixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IFloor}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IFloor(&mut self, ctx: &IFloorContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code IAtom}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_IAtom(&mut self, ctx: &IAtomContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ICeiling}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_ICeiling(&mut self, ctx: &ICeilingContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ISlice}
     * labeled alternative in {@link ValkyrieAntlrParser#inline_expression}.
     * @param ctx the parse tree
     */
    fn visit_ISlice(&mut self, ctx: &ISliceContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TPrefix}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TPrefix(&mut self, ctx: &TPrefixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TGeneric}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TGeneric(&mut self, ctx: &TGenericContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TPattern}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TPattern(&mut self, ctx: &TPatternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TTuple}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TTuple(&mut self, ctx: &TTupleContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TMust}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TMust(&mut self, ctx: &TMustContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TAtom}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TAtom(&mut self, ctx: &TAtomContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TAdd}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TAdd(&mut self, ctx: &TAddContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TArrows}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TArrows(&mut self, ctx: &TArrowsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code TOptional}
     * labeled alternative in {@link ValkyrieAntlrParser#type_expression}.
     * @param ctx the parse tree
     */
    fn visit_TOptional(&mut self, ctx: &TOptionalContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code AString}
     * labeled alternative in {@link ValkyrieAntlrParser#atomic}.
     * @param ctx the parse tree
     */
    fn visit_AString(&mut self, ctx: &AStringContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ANumber}
     * labeled alternative in {@link ValkyrieAntlrParser#atomic}.
     * @param ctx the parse tree
     */
    fn visit_ANumber(&mut self, ctx: &ANumberContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ALambda}
     * labeled alternative in {@link ValkyrieAntlrParser#atomic}.
     * @param ctx the parse tree
     */
    fn visit_ALambda(&mut self, ctx: &ALambdaContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code AOutput}
     * labeled alternative in {@link ValkyrieAntlrParser#atomic}.
     * @param ctx the parse tree
     */
    fn visit_AOutput(&mut self, ctx: &AOutputContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ANamepath}
     * labeled alternative in {@link ValkyrieAntlrParser#atomic}.
     * @param ctx the parse tree
     */
    fn visit_ANamepath(&mut self, ctx: &ANamepathContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ASpecial}
     * labeled alternative in {@link ValkyrieAntlrParser#atomic}.
     * @param ctx the parse tree
     */
    fn visit_ASpecial(&mut self, ctx: &ASpecialContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CReturn}
     * labeled alternative in {@link ValkyrieAntlrParser#control_expression}.
     * @param ctx the parse tree
     */
    fn visit_CReturn(&mut self, ctx: &CReturnContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CBreak}
     * labeled alternative in {@link ValkyrieAntlrParser#control_expression}.
     * @param ctx the parse tree
     */
    fn visit_CBreak(&mut self, ctx: &CBreakContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CContinue}
     * labeled alternative in {@link ValkyrieAntlrParser#control_expression}.
     * @param ctx the parse tree
     */
    fn visit_CContinue(&mut self, ctx: &CContinueContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CRaise}
     * labeled alternative in {@link ValkyrieAntlrParser#control_expression}.
     * @param ctx the parse tree
     */
    fn visit_CRaise(&mut self, ctx: &CRaiseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CYield}
     * labeled alternative in {@link ValkyrieAntlrParser#control_expression}.
     * @param ctx the parse tree
     */
    fn visit_CYield(&mut self, ctx: &CYieldContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CWith}
     * labeled alternative in {@link ValkyrieAntlrParser#control_expression}.
     * @param ctx the parse tree
     */
    fn visit_CWith(&mut self, ctx: &CWithContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CFall}
     * labeled alternative in {@link ValkyrieAntlrParser#control_expression}.
     * @param ctx the parse tree
     */
    fn visit_CFall(&mut self, ctx: &CFallContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#op_prefix}.
     * @param ctx the parse tree
     */
    fn visit_op_prefix(&mut self, ctx: &Op_prefixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#op_suffix}.
     * @param ctx the parse tree
     */
    fn visit_op_suffix(&mut self, ctx: &Op_suffixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#op_compare}.
     * @param ctx the parse tree
     */
    fn visit_op_compare(&mut self, ctx: &Op_compareContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#op_pattern}.
     * @param ctx the parse tree
     */
    fn visit_op_pattern(&mut self, ctx: &Op_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#infix_map}.
     * @param ctx the parse tree
     */
    fn visit_infix_map(&mut self, ctx: &Infix_mapContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#infix_pow}.
     * @param ctx the parse tree
     */
    fn visit_infix_pow(&mut self, ctx: &Infix_powContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#infix_range}.
     * @param ctx the parse tree
     */
    fn visit_infix_range(&mut self, ctx: &Infix_rangeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#infix_arrows}.
     * @param ctx the parse tree
     */
    fn visit_infix_arrows(&mut self, ctx: &Infix_arrowsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#op_multiple}.
     * @param ctx the parse tree
     */
    fn visit_op_multiple(&mut self, ctx: &Op_multipleContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#op_plus}.
     * @param ctx the parse tree
     */
    fn visit_op_plus(&mut self, ctx: &Op_plusContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#op_logic}.
     * @param ctx the parse tree
     */
    fn visit_op_logic(&mut self, ctx: &Op_logicContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#op_pipeline}.
     * @param ctx the parse tree
     */
    fn visit_op_pipeline(&mut self, ctx: &Op_pipelineContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#infix_assign}.
     * @param ctx the parse tree
     */
    fn visit_infix_assign(&mut self, ctx: &Infix_assignContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#infix_is}.
     * @param ctx the parse tree
     */
    fn visit_infix_is(&mut self, ctx: &Infix_isContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#infix_as}.
     * @param ctx the parse tree
     */
    fn visit_infix_as(&mut self, ctx: &Infix_asContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#infix_in}.
     * @param ctx the parse tree
     */
    fn visit_infix_in(&mut self, ctx: &Infix_inContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_generic}.
     * @param ctx the parse tree
     */
    fn visit_define_generic(&mut self, ctx: &Define_genericContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#generic_item}.
     * @param ctx the parse tree
     */
    fn visit_generic_item(&mut self, ctx: &Generic_itemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#generic_call}.
     * @param ctx the parse tree
     */
    fn visit_generic_call(&mut self, ctx: &Generic_callContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#generic_call_in_type}.
     * @param ctx the parse tree
     */
    fn visit_generic_call_in_type(&mut self, ctx: &Generic_call_in_typeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#generic_pair}.
     * @param ctx the parse tree
     */
    fn visit_generic_pair(&mut self, ctx: &Generic_pairContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#define_label}.
     * @param ctx the parse tree
     */
    fn visit_define_label(&mut self, ctx: &Define_labelContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#template_call}.
     * @param ctx the parse tree
     */
    fn visit_template_call(&mut self, ctx: &Template_callContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#template_block}.
     * @param ctx the parse tree
     */
    fn visit_template_block(&mut self, ctx: &Template_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#template_statements}.
     * @param ctx the parse tree
     */
    fn visit_template_statements(&mut self, ctx: &Template_statementsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#template_implements}.
     * @param ctx the parse tree
     */
    fn visit_template_implements(&mut self, ctx: &Template_implementsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#where_block}.
     * @param ctx the parse tree
     */
    fn visit_where_block(&mut self, ctx: &Where_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#where_bound}.
     * @param ctx the parse tree
     */
    fn visit_where_bound(&mut self, ctx: &Where_boundContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#require_block}.
     * @param ctx the parse tree
     */
    fn visit_require_block(&mut self, ctx: &Require_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#macro_call}.
     * @param ctx the parse tree
     */
    fn visit_macro_call(&mut self, ctx: &Macro_callContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#annotation}.
     * @param ctx the parse tree
     */
    fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#annotation_call_item}.
     * @param ctx the parse tree
     */
    fn visit_annotation_call_item(&mut self, ctx: &Annotation_call_itemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#try_statement}.
     * @param ctx the parse tree
     */
    fn visit_try_statement(&mut self, ctx: &Try_statementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#match_statement}.
     * @param ctx the parse tree
     */
    fn visit_match_statement(&mut self, ctx: &Match_statementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#match_block}.
     * @param ctx the parse tree
     */
    fn visit_match_block(&mut self, ctx: &Match_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code MatchWith}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn visit_MatchWith(&mut self, ctx: &MatchWithContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code MatchWithMany}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn visit_MatchWithMany(&mut self, ctx: &MatchWithManyContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code MatchType}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn visit_MatchType(&mut self, ctx: &MatchTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code MatchWhen}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn visit_MatchWhen(&mut self, ctx: &MatchWhenContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code MatchElse}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn visit_MatchElse(&mut self, ctx: &MatchElseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code MatchCase}
     * labeled alternative in {@link ValkyrieAntlrParser#match_terms}.
     * @param ctx the parse tree
     */
    fn visit_MatchCase(&mut self, ctx: &MatchCaseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#match_case_block}.
     * @param ctx the parse tree
     */
    fn visit_match_case_block(&mut self, ctx: &Match_case_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CaseOR}
     * labeled alternative in {@link ValkyrieAntlrParser#case_pattern}.
     * @param ctx the parse tree
     */
    fn visit_CaseOR(&mut self, ctx: &CaseORContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CaseAtom}
     * labeled alternative in {@link ValkyrieAntlrParser#case_pattern}.
     * @param ctx the parse tree
     */
    fn visit_CaseAtom(&mut self, ctx: &CaseAtomContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CaseUntil}
     * labeled alternative in {@link ValkyrieAntlrParser#case_pattern}.
     * @param ctx the parse tree
     */
    fn visit_CaseUntil(&mut self, ctx: &CaseUntilContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#case_pattern_item}.
     * @param ctx the parse tree
     */
    fn visit_case_pattern_item(&mut self, ctx: &Case_pattern_itemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#case_pattern_tuple}.
     * @param ctx the parse tree
     */
    fn visit_case_pattern_tuple(&mut self, ctx: &Case_pattern_tupleContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#object_statement}.
     * @param ctx the parse tree
     */
    fn visit_object_statement(&mut self, ctx: &Object_statementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#new_statement}.
     * @param ctx the parse tree
     */
    fn visit_new_statement(&mut self, ctx: &New_statementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#new_block}.
     * @param ctx the parse tree
     */
    fn visit_new_block(&mut self, ctx: &New_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#new_call_item}.
     * @param ctx the parse tree
     */
    fn visit_new_call_item(&mut self, ctx: &New_call_itemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NK1}
     * labeled alternative in {@link ValkyrieAntlrParser#new_call_key}.
     * @param ctx the parse tree
     */
    fn visit_NK1(&mut self, ctx: &NK1Context<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NK2}
     * labeled alternative in {@link ValkyrieAntlrParser#new_call_key}.
     * @param ctx the parse tree
     */
    fn visit_NK2(&mut self, ctx: &NK2Context<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NK3}
     * labeled alternative in {@link ValkyrieAntlrParser#new_call_key}.
     * @param ctx the parse tree
     */
    fn visit_NK3(&mut self, ctx: &NK3Context<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NK4}
     * labeled alternative in {@link ValkyrieAntlrParser#new_call_key}.
     * @param ctx the parse tree
     */
    fn visit_NK4(&mut self, ctx: &NK4Context<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#tuple_literal}.
     * @param ctx the parse tree
     */
    fn visit_tuple_literal(&mut self, ctx: &Tuple_literalContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#collection_pair}.
     * @param ctx the parse tree
     */
    fn visit_collection_pair(&mut self, ctx: &Collection_pairContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CK1}
     * labeled alternative in {@link ValkyrieAntlrParser#collection_key}.
     * @param ctx the parse tree
     */
    fn visit_CK1(&mut self, ctx: &CK1Context<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CK2}
     * labeled alternative in {@link ValkyrieAntlrParser#collection_key}.
     * @param ctx the parse tree
     */
    fn visit_CK2(&mut self, ctx: &CK2Context<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code CK3}
     * labeled alternative in {@link ValkyrieAntlrParser#collection_key}.
     * @param ctx the parse tree
     */
    fn visit_CK3(&mut self, ctx: &CK3Context<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#slice_call}.
     * @param ctx the parse tree
     */
    fn visit_slice_call(&mut self, ctx: &Slice_callContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#range_literal}.
     * @param ctx the parse tree
     */
    fn visit_range_literal(&mut self, ctx: &Range_literalContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#range_axis}.
     * @param ctx the parse tree
     */
    fn visit_range_axis(&mut self, ctx: &Range_axisContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#modifiers}.
     * @param ctx the parse tree
     */
    fn visit_modifiers(&mut self, ctx: &ModifiersContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#modified_identifier}.
     * @param ctx the parse tree
     */
    fn visit_modified_identifier(&mut self, ctx: &Modified_identifierContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#modified_namepath}.
     * @param ctx the parse tree
     */
    fn visit_modified_namepath(&mut self, ctx: &Modified_namepathContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#lambda_name}.
     * @param ctx the parse tree
     */
    fn visit_lambda_name(&mut self, ctx: &Lambda_nameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code PositiveOutput}
     * labeled alternative in {@link ValkyrieAntlrParser#output_name}.
     * @param ctx the parse tree
     */
    fn visit_PositiveOutput(&mut self, ctx: &PositiveOutputContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code NegativeOutput}
     * labeled alternative in {@link ValkyrieAntlrParser#output_name}.
     * @param ctx the parse tree
     */
    fn visit_NegativeOutput(&mut self, ctx: &NegativeOutputContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#namepath_free}.
     * @param ctx the parse tree
     */
    fn visit_namepath_free(&mut self, ctx: &Namepath_freeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#namepath}.
     * @param ctx the parse tree
     */
    fn visit_namepath(&mut self, ctx: &NamepathContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#identifier}.
     * @param ctx the parse tree
     */
    fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#number}.
     * @param ctx the parse tree
     */
    fn visit_number(&mut self, ctx: &NumberContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#number_literal}.
     * @param ctx the parse tree
     */
    fn visit_number_literal(&mut self, ctx: &Number_literalContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#string}.
     * @param ctx the parse tree
     */
    fn visit_string(&mut self, ctx: &StringContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link ValkyrieAntlrParser#string_literal}.
     * @param ctx the parse tree
     */
    fn visit_string_literal(&mut self, ctx: &String_literalContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }
}

impl<'input, T> ValkyrieAntlrVisitor<'input> for T
where
    T: ValkyrieAntlrVisitorCompat<'input>,
{
    fn visit_program(&mut self, ctx: &ProgramContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_program(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_eos(&mut self, ctx: &EosContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_eos(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_eos_free(&mut self, ctx: &Eos_freeContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_eos_free(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_namespace(&mut self, ctx: &Define_namespaceContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_define_namespace(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_import_statement(&mut self, ctx: &Import_statementContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_import_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_import_as(&mut self, ctx: &Import_asContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_import_as(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_import_term(&mut self, ctx: &Import_termContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_import_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_import_name(&mut self, ctx: &Import_nameContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_import_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_import_block(&mut self, ctx: &Import_blockContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_import_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_extension(&mut self, ctx: &Define_extensionContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_define_extension(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_class(&mut self, ctx: &Define_classContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_define_class(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_class_block(&mut self, ctx: &Class_blockContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_class_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_class_inherit(&mut self, ctx: &Class_inheritContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_class_inherit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_class_inherit_item(&mut self, ctx: &Class_inherit_itemContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_class_inherit_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_class_field(&mut self, ctx: &Class_fieldContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_class_field(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_class_method(&mut self, ctx: &Class_methodContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_class_method(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_class_dsl(&mut self, ctx: &Class_dslContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_class_dsl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_trait(&mut self, ctx: &Define_traitContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_define_trait(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_trait_block(&mut self, ctx: &Trait_blockContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_trait_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_trait_type(&mut self, ctx: &Define_trait_typeContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_define_trait_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_extends(&mut self, ctx: &Define_extendsContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_define_extends(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_extends_block(&mut self, ctx: &Extends_blockContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_extends_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_with_implements(&mut self, ctx: &With_implementsContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_with_implements(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_union(&mut self, ctx: &Define_unionContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_define_union(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_base_layout(&mut self, ctx: &Base_layoutContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_base_layout(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_union_block(&mut self, ctx: &Union_blockContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_union_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_variant(&mut self, ctx: &Define_variantContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_define_variant(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_variant_block(&mut self, ctx: &Variant_blockContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_variant_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_bitflags(&mut self, ctx: &Define_bitflagsContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_define_bitflags(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_bitflags_block(&mut self, ctx: &Bitflags_blockContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_bitflags_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_bitflags_item(&mut self, ctx: &Bitflags_itemContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_bitflags_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_function(&mut self, ctx: &Define_functionContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_define_function(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_function_parameters(&mut self, ctx: &Function_parametersContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_function_parameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_parameter_item(&mut self, ctx: &Parameter_itemContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_parameter_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_DictParameters(&mut self, ctx: &DictParametersContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_DictParameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ListParameters(&mut self, ctx: &ListParametersContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ListParameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ContextParameter(&mut self, ctx: &ContextParameterContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ContextParameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_return_type(&mut self, ctx: &Return_typeContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_return_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_parameter_default(&mut self, ctx: &Parameter_defaultContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_parameter_default(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_function_call(&mut self, ctx: &Function_callContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_function_call(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_NormalClosure(&mut self, ctx: &NormalClosureContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_NormalClosure(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_SlotClosure(&mut self, ctx: &SlotClosureContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_SlotClosure(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_IntegerClosure(&mut self, ctx: &IntegerClosureContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_IntegerClosure(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_InternalClosure(&mut self, ctx: &InternalClosureContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_InternalClosure(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_tuple_call_body(&mut self, ctx: &Tuple_call_bodyContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_tuple_call_body(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_tuple_call_item(&mut self, ctx: &Tuple_call_itemContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_tuple_call_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_lambda(&mut self, ctx: &Define_lambdaContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_define_lambda(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_function_block(&mut self, ctx: &Function_blockContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_function_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_let_binding(&mut self, ctx: &Let_bindingContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_let_binding(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_let_pattern(&mut self, ctx: &Let_patternContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_let_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_let_pattern_plain(&mut self, ctx: &Let_pattern_plainContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_let_pattern_plain(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_let_pattern_tuple(&mut self, ctx: &Let_pattern_tupleContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_let_pattern_tuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_let_pattern_item(&mut self, ctx: &Let_pattern_itemContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_let_pattern_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_type(&mut self, ctx: &Define_typeContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_define_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_type_hint(&mut self, ctx: &Type_hintContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_type_hint(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_if_statement(&mut self, ctx: &If_statementContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_if_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_guard_statement(&mut self, ctx: &Guard_statementContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_guard_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_else_if_statement(&mut self, ctx: &Else_if_statementContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_else_if_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_WhileLoop(&mut self, ctx: &WhileLoopContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_WhileLoop(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_WhileLet(&mut self, ctx: &WhileLetContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_WhileLet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ForLoop(&mut self, ctx: &ForLoopContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ForLoop(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_if_guard(&mut self, ctx: &If_guardContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_if_guard(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_expression_root(&mut self, ctx: &Expression_rootContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_expression_root(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EIf(&mut self, ctx: &EIfContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EIf(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EUntil(&mut self, ctx: &EUntilContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EUntil(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EIn(&mut self, ctx: &EInContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EIn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EFloor(&mut self, ctx: &EFloorContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EFloor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EPlus(&mut self, ctx: &EPlusContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EPlus(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ESlice(&mut self, ctx: &ESliceContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ESlice(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EDotMatch(&mut self, ctx: &EDotMatchContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EDotMatch(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ETry(&mut self, ctx: &ETryContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ETry(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EAs(&mut self, ctx: &EAsContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EAs(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EObject(&mut self, ctx: &EObjectContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EObject(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ETuple(&mut self, ctx: &ETupleContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ETuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ERange(&mut self, ctx: &ERangeContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ERange(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EAssign(&mut self, ctx: &EAssignContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EAssign(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EAtom(&mut self, ctx: &EAtomContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EAtom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EMul(&mut self, ctx: &EMulContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EMul(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EPipe(&mut self, ctx: &EPipeContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EPipe(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ELambda(&mut self, ctx: &ELambdaContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ELambda(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EOrElse(&mut self, ctx: &EOrElseContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EOrElse(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EGroup(&mut self, ctx: &EGroupContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EGroup(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EFunction(&mut self, ctx: &EFunctionContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ESuffix(&mut self, ctx: &ESuffixContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ESuffix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EClosure(&mut self, ctx: &EClosureContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EClosure(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EMatch(&mut self, ctx: &EMatchContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EMatch(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EMacro(&mut self, ctx: &EMacroContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EMacro(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EPrefix(&mut self, ctx: &EPrefixContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EPrefix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ECompare(&mut self, ctx: &ECompareContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ECompare(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EGeneric(&mut self, ctx: &EGenericContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EGeneric(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EIsA(&mut self, ctx: &EIsAContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EIsA(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EPow(&mut self, ctx: &EPowContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EPow(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EDefine(&mut self, ctx: &EDefineContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EDefine(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EMap(&mut self, ctx: &EMapContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EMap(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ELogic(&mut self, ctx: &ELogicContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ELogic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_EControl(&mut self, ctx: &EControlContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_EControl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ENew(&mut self, ctx: &ENewContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ENew(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ECeiling(&mut self, ctx: &ECeilingContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ECeiling(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ILogic(&mut self, ctx: &ILogicContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ILogic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_IPlus(&mut self, ctx: &IPlusContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_IPlus(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ICompare(&mut self, ctx: &ICompareContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ICompare(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_IGeneric(&mut self, ctx: &IGenericContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_IGeneric(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_IFunction(&mut self, ctx: &IFunctionContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_IFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_IMap(&mut self, ctx: &IMapContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_IMap(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_IIs(&mut self, ctx: &IIsContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_IIs(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ITuple(&mut self, ctx: &ITupleContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ITuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_IRange(&mut self, ctx: &IRangeContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_IRange(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_IGroup(&mut self, ctx: &IGroupContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_IGroup(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_IMul(&mut self, ctx: &IMulContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_IMul(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_IAs(&mut self, ctx: &IAsContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_IAs(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_IPrefix(&mut self, ctx: &IPrefixContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_IPrefix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_IFloor(&mut self, ctx: &IFloorContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_IFloor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_IAtom(&mut self, ctx: &IAtomContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_IAtom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ICeiling(&mut self, ctx: &ICeilingContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ICeiling(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ISlice(&mut self, ctx: &ISliceContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ISlice(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_TPrefix(&mut self, ctx: &TPrefixContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_TPrefix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_TGeneric(&mut self, ctx: &TGenericContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_TGeneric(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_TPattern(&mut self, ctx: &TPatternContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_TPattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_TTuple(&mut self, ctx: &TTupleContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_TTuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_TMust(&mut self, ctx: &TMustContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_TMust(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_TAtom(&mut self, ctx: &TAtomContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_TAtom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_TAdd(&mut self, ctx: &TAddContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_TAdd(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_TArrows(&mut self, ctx: &TArrowsContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_TArrows(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_TOptional(&mut self, ctx: &TOptionalContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_TOptional(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_AString(&mut self, ctx: &AStringContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_AString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ANumber(&mut self, ctx: &ANumberContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ANumber(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ALambda(&mut self, ctx: &ALambdaContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ALambda(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_AOutput(&mut self, ctx: &AOutputContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_AOutput(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ANamepath(&mut self, ctx: &ANamepathContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ANamepath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ASpecial(&mut self, ctx: &ASpecialContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_ASpecial(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CReturn(&mut self, ctx: &CReturnContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_CReturn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CBreak(&mut self, ctx: &CBreakContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_CBreak(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CContinue(&mut self, ctx: &CContinueContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_CContinue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CRaise(&mut self, ctx: &CRaiseContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_CRaise(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CYield(&mut self, ctx: &CYieldContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_CYield(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CWith(&mut self, ctx: &CWithContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_CWith(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CFall(&mut self, ctx: &CFallContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_CFall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_op_prefix(&mut self, ctx: &Op_prefixContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_op_prefix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_op_suffix(&mut self, ctx: &Op_suffixContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_op_suffix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_op_compare(&mut self, ctx: &Op_compareContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_op_compare(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_op_pattern(&mut self, ctx: &Op_patternContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_op_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_infix_map(&mut self, ctx: &Infix_mapContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_infix_map(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_infix_pow(&mut self, ctx: &Infix_powContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_infix_pow(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_infix_range(&mut self, ctx: &Infix_rangeContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_infix_range(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_infix_arrows(&mut self, ctx: &Infix_arrowsContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_infix_arrows(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_op_multiple(&mut self, ctx: &Op_multipleContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_op_multiple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_op_plus(&mut self, ctx: &Op_plusContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_op_plus(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_op_logic(&mut self, ctx: &Op_logicContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_op_logic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_op_pipeline(&mut self, ctx: &Op_pipelineContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_op_pipeline(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_infix_assign(&mut self, ctx: &Infix_assignContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_infix_assign(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_infix_is(&mut self, ctx: &Infix_isContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_infix_is(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_infix_as(&mut self, ctx: &Infix_asContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_infix_as(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_infix_in(&mut self, ctx: &Infix_inContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_infix_in(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_generic(&mut self, ctx: &Define_genericContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_define_generic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_generic_item(&mut self, ctx: &Generic_itemContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_generic_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_generic_call(&mut self, ctx: &Generic_callContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_generic_call(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_generic_call_in_type(&mut self, ctx: &Generic_call_in_typeContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_generic_call_in_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_generic_pair(&mut self, ctx: &Generic_pairContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_generic_pair(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_label(&mut self, ctx: &Define_labelContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_define_label(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_template_call(&mut self, ctx: &Template_callContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_template_call(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_template_block(&mut self, ctx: &Template_blockContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_template_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_template_statements(&mut self, ctx: &Template_statementsContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_template_statements(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_template_implements(&mut self, ctx: &Template_implementsContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_template_implements(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_where_block(&mut self, ctx: &Where_blockContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_where_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_where_bound(&mut self, ctx: &Where_boundContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_where_bound(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_require_block(&mut self, ctx: &Require_blockContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_require_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_macro_call(&mut self, ctx: &Macro_callContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_macro_call(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_annotation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_annotation_call_item(&mut self, ctx: &Annotation_call_itemContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_annotation_call_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_try_statement(&mut self, ctx: &Try_statementContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_try_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_match_statement(&mut self, ctx: &Match_statementContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_match_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_match_block(&mut self, ctx: &Match_blockContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_match_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_MatchWith(&mut self, ctx: &MatchWithContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_MatchWith(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_MatchWithMany(&mut self, ctx: &MatchWithManyContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_MatchWithMany(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_MatchType(&mut self, ctx: &MatchTypeContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_MatchType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_MatchWhen(&mut self, ctx: &MatchWhenContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_MatchWhen(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_MatchElse(&mut self, ctx: &MatchElseContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_MatchElse(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_MatchCase(&mut self, ctx: &MatchCaseContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_MatchCase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_match_case_block(&mut self, ctx: &Match_case_blockContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_match_case_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CaseOR(&mut self, ctx: &CaseORContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_CaseOR(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CaseAtom(&mut self, ctx: &CaseAtomContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_CaseAtom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CaseUntil(&mut self, ctx: &CaseUntilContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_CaseUntil(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_case_pattern_item(&mut self, ctx: &Case_pattern_itemContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_case_pattern_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_case_pattern_tuple(&mut self, ctx: &Case_pattern_tupleContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_case_pattern_tuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_object_statement(&mut self, ctx: &Object_statementContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_object_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_new_statement(&mut self, ctx: &New_statementContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_new_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_new_block(&mut self, ctx: &New_blockContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_new_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_new_call_item(&mut self, ctx: &New_call_itemContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_new_call_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_NK1(&mut self, ctx: &NK1Context<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_NK1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_NK2(&mut self, ctx: &NK2Context<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_NK2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_NK3(&mut self, ctx: &NK3Context<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_NK3(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_NK4(&mut self, ctx: &NK4Context<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_NK4(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_tuple_literal(&mut self, ctx: &Tuple_literalContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_tuple_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_collection_pair(&mut self, ctx: &Collection_pairContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_collection_pair(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CK1(&mut self, ctx: &CK1Context<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_CK1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CK2(&mut self, ctx: &CK2Context<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_CK2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CK3(&mut self, ctx: &CK3Context<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_CK3(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_slice_call(&mut self, ctx: &Slice_callContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_slice_call(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_range_literal(&mut self, ctx: &Range_literalContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_range_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_range_axis(&mut self, ctx: &Range_axisContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_range_axis(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_modifiers(&mut self, ctx: &ModifiersContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_modifiers(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_modified_identifier(&mut self, ctx: &Modified_identifierContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_modified_identifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_modified_namepath(&mut self, ctx: &Modified_namepathContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_modified_namepath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_lambda_name(&mut self, ctx: &Lambda_nameContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_lambda_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_PositiveOutput(&mut self, ctx: &PositiveOutputContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_PositiveOutput(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_NegativeOutput(&mut self, ctx: &NegativeOutputContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_NegativeOutput(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_namepath_free(&mut self, ctx: &Namepath_freeContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_namepath_free(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_namepath(&mut self, ctx: &NamepathContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_namepath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_identifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_number(&mut self, ctx: &NumberContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_number(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_number_literal(&mut self, ctx: &Number_literalContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_number_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_string(&mut self, ctx: &StringContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_string(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_string_literal(&mut self, ctx: &String_literalContext<'input>) {
        let result = <Self as ValkyrieAntlrVisitorCompat>::visit_string_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }
}
