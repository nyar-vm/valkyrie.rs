grammar ValkyrieAntlr;
import ValkyrieBasic;
options {
	language = Java;
}

// $antlr-format useTab false, columnLimit 144
// $antlr-format alignColons hanging, alignSemicolons hanging, alignFirstTokens true
program: (top_statement | eos)* EOF;
top_statement
    : define_namespace   # SNamespace
    | import_statement   # SImport
    | define_extension   # SExtension
    | define_class       # SClass
    | define_union       # SUnion
    | define_bitflags    # SFlags
    | define_trait       # STrait
    | define_extends     # SExtends
    | define_function    # SFunction
    | function_statement # S1
    ;
eos:      SEMICOLON;
eos_free: COMMA | SEMICOLON;
// ===========================================================================
define_namespace: KW_NAMESPACE namepath_free eos?;
// ===========================================================================
import_statement: KW_IMPORT import_term;
import_as:        KW_AS (OP_AT | OP_HASH)? identifier;
import_term
    : import_block
    | OP_AT import_name import_as?
    | OP_AT import_name ((OP_PROPORTION | DOT)? import_block)?
    | OP_HASH import_name import_as?
    | OP_HASH import_name ((OP_PROPORTION | DOT)? import_block)?
    | import_name import_as?
    | import_name ((OP_PROPORTION | DOT)? import_block)?
    | eos_free
    ;
import_name:  identifier ((OP_PROPORTION | DOT) identifier)* ((OP_PROPORTION | DOT) (OP_MUL));
import_block: BRACE_L BRACE_R | BRACE_L import_term* BRACE_R;
// ===========================================================================
define_extension: KW_EXTENSION;
// ===========================================================================
define_class
    : template_call? annotation* modifiers KW_CLASS identifier define_generic? class_inherit? type_hint? class_block eos?
    ;
class_block: BRACE_L (class_dsl | class_method | class_field | eos_free)* BRACE_R;
class_inherit
    : PARENTHESES_L PARENTHESES_R
    | PARENTHESES_L class_inherit_item (COMMA class_inherit_item)* COMMA? PARENTHESES_R
    ;
class_inherit_item: modified_namepath;
class_field:        annotation* modified_identifier type_hint? parameter_default?;
class_method
    : annotation* modified_namepath define_generic? function_parameters type_hint? effect_hint? function_block?
    ;
class_dsl: annotation* modified_identifier class_block;
// ===========================================================================
define_trait
    : template_call? annotation* modifiers KW_TRAIT identifier define_generic? with_implements? trait_block eos?
    ;
trait_block:       BRACE_L (define_trait_type | class_method | class_field | eos_free)* BRACE_R;
define_trait_type: KW_TYPE identifier (OP_ASSIGN type_expression)?;
// ===========================================================================
define_extends
    : template_call? annotation* modifiers KW_EXTENDS namepath define_generic? with_implements? trait_block
    ;
with_implements: (COLON | KW_IMPLEMENTS) type_expression;
// ===========================================================================
define_union:   annotation* modifiers KW_UNION identifier base_layout? type_hint? union_block;
base_layout:    PARENTHESES_L type_expression? PARENTHESES_R;
union_block:    BRACE_L (class_method | define_variant | eos_free)* BRACE_R;
define_variant: identifier variant_block?;
variant_block:  BRACE_L (class_field | eos_free)* BRACE_R;
// ===========================================================================
define_bitflags
    : annotation* modifiers KW_BITFLAGS identifier base_layout? type_hint? bitflags_block
    ;
bitflags_block: BRACE_L (class_method | bitflags_item | eos_free)* BRACE_R;
bitflags_item:  annotation* identifier (OP_ASSIGN expression)?;
// ===========================================================================
define_function
    : template_call? annotation* modifiers KW_FUNCTION namepath define_generic? function_parameters type_hint? effect_hint? function_block
    ;
function_parameters
    : PARENTHESES_L PARENTHESES_R
    | PARENTHESES_L parameter_item (COMMA parameter_item)* PARENTHESES_R
    ;
parameter_item:    annotation* modified_identifier type_hint? parameter_default?;
parameter_default: OP_ASSIGN expression;
// ===========================================================================
function_call
    : OP_THROW? tuple_call_body // method?(b)
    ;
dot_call
    : OP_THROW? DOT namepath tuple_call_body? // ?.method()
    | OP_THROW? DOT OP_AT? namepath tuple_call_body? // ?.@method()
    ;
tuple_call_body
    : PARENTHESES_L PARENTHESES_R
    | PARENTHESES_L tuple_call_item (COMMA tuple_call_item)* COMMA? PARENTHESES_R
    ;
tuple_call_item: identifier COLON expression | expression;
// ===========================================================================
define_lambda: annotation* KW_LAMBDA function_parameters type_hint? function_block;
lambda_call:   OP_THROW? function_block;
// ===========================================================================
function_block: BRACE_L (function_statement | eos)* BRACE_R;
function_statement
    : define_type     # SType
    | define_lambda   # Slambda
    | let_binding     # SLet
    | loop_statement  # SLoop
    | guard_statement # SIfLet
    | expression_root # S2
    ;
// ===========================================================================
let_binding:       KW_LET let_pattern type_hint? (OP_ASSIGN expression)?;
let_pattern:       let_pattern_tuple | let_pattern_plain;
let_pattern_plain: modified_identifier (COMMA modified_identifier)* COMMA?;
let_pattern_tuple
    : PARENTHESES_L (
        let_pattern_item COMMA
        | let_pattern_item (COMMA let_pattern_item)+ COMMA?
    )? PARENTHESES_R
    | namepath PARENTHESES_L (let_pattern_item (COMMA let_pattern_item)* COMMA?)? PARENTHESES_R
    | namepath? BRACKET_L (let_pattern_item (COMMA let_pattern_item)* COMMA?)? BRACKET_R
    | namepath? BRACE_L (let_pattern_item (COMMA let_pattern_item)* COMMA?)? BRACE_R
    ;
let_pattern_item
    : (modified_identifier COLON)? (bind = identifier OP_BIND)? let_pattern_tuple
    | (modified_identifier COLON)? (bind = identifier OP_BIND)? identifier
    | modified_identifier? OP_DECONSTRUCT bind = identifier?
    | modified_identifier
    ;
// ===========================================================================
define_type: KW_TYPE identifier OP_ASSIGN identifier;
type_hint:   (COLON | OP_ARROW) type_expression;
effect_hint: OP_DIV type_expression;
// ===========================================================================
if_statement
    : annotation* KW_IF inline_expression then_block = function_block else_if_statement* (
        KW_ELSE else_block = function_block
    )?
    ;
guard_statement
    : annotation* KW_IF (KW_LET | KW_NOT) (let_pattern_tuple | identifier | SPECIAL) OP_ASSIGN inline_expression then = function_block
    | annotation* KW_IF (KW_LET | KW_NOT) inline_expression then = function_block
    ;
else_if_statement: KW_ELSE KW_IF inline_expression function_block;
// ===========================================================================
loop_statement
    : annotation* KW_WHILE inline_expression function_block                              # WhileLoop
    | annotation* KW_WHILE KW_LET let_pattern OP_ASSIGN inline_expression function_block # WhileLet
    | annotation* KW_FOR let_pattern infix_in inline_expression if_guard? function_block # ForLoop
    ;
if_guard: KW_IF inline_expression;
// ==========================================================================
expression_root: annotation* expression eos?;
expression
    : <assoc = right> lhs = expression OP_POW rhs = expression # EPow
    | lhs = expression op_multiple rhs = expression            # EMul
    | lhs = expression op_plus rhs = expression                # EPlus
    | lhs = expression op_logic rhs = expression               # ELogic
    | lhs = expression op_compare rhs = expression             # ECompare
    | lhs = expression OP_UNTIL rhs = expression               # EUntil
    | lhs = expression infix_is rhs = type_expression          # EIsA
    | lhs = expression KW_AS rhs = type_expression             # EAs
    | lhs = expression infix_in rhs = expression               # EIn
    | lhs = expression op_assign rhs = type_expression         # EAssign
    | lhs = expression OP_OR_ELSE rhs = type_expression        # EOrElse
    | lhs = expression op_pipeline rhs = type_expression       # EPipe
    // suffix
    | expression op_suffix     # ESuffix
    | expression slice_call    # ESlice
    | expression offset_call   # EOffset
    | expression generic_call  # EGeneric
    | expression lambda_call   # ELambda
    | expression match_call    # EDotMatch
    | expression function_call # EDotFunction
    | expression dot_call      # EDot
    // suffix
    | op_prefix expression # EPrefix
    // atomic
    | PARENTHESES_L expression PARENTHESES_R # EGroup
    | control_expression                     # EControl
    | term_with_follow                       # E1
    ;
// statement without return
term_with_follow
    : if_statement     # SIf
    | new_statement    # ENew
    | try_statement    # ETry
    | match_statement  # EMatch
    | object_statement # EObject
    | macro_call       # EMacro
    | function_call    # EFunction
    | define_label     # EDefine
    | tuple_literal    # ETuple
    | range_literal    # ERange
    | atomic           # E2
    ;
// statement with return type
inline_expression
    : lhs = inline_expression op_multiple rhs = inline_expression # IMul
    | lhs = inline_expression op_plus rhs = inline_expression     # IPlus
    | lhs = inline_expression op_logic rhs = inline_expression    # ILogic
    | lhs = inline_expression op_compare rhs = inline_expression  # ICompare
    | lhs = inline_expression infix_is rhs = inline_expression    # IIsA
    | lhs = inline_expression OP_UNTIL rhs = inline_expression    # IRange
    | lhs = inline_expression KW_AS rhs = inline_expression       # IAs
    // suffix
    | inline_expression dot_call   # IDot
    | inline_expression slice_call # ISlice
    // prefix
    | op_prefix inline_expression # IPrefix
    | atomic                      # E3
    ;
type_expression
    : type_expression op_pattern type_expression   # TPattern
    | type_expression infix_arrows type_expression # TArrows
    | type_expression OP_ADD type_expression       # TAdd
    | type_expression generic_call_in_type         # TGeneric
    | PARENTHESES_L (
        type_expression COMMA // must add ,
        | type_expression (COMMA type_expression)+ COMMA?
    )? PARENTHESES_R # TTuple
    | atomic         # E4
    ;
atomic
    : string_literal # AString
    | number_literal # ANumber
    | lambda_name    # ALambda
    | namepath       # ANamepath
    | SPECIAL        # ASpecial
    ;
// ===========================================================================
op_prefix
    : OP_NOT
    | OP_ADD
    | OP_SUB
    | OP_AND
    | OP_REFERENCE
    | OP_DECONSTRUCT
    | OP_INVERSE
    | OP_ROOTS
    | OP_MUL
    ;
op_suffix: OP_NOT | OP_TEMPERATURE | OP_TRANSPOSE | OP_PERCENT;
control_expression
    : (RETURN | RESUME expression?)            # CReturn
    | BREAK (OP_LABEL identifier)?             # CBreak
    | CONTINUE (OP_LABEL identifier)?          # CContinue
    | RAISE expression                         # CRaise
    | YIELD (OP_LABEL identifier)? expression? # CYield
    | YIELD BREAK                              # CBreak
    | YIELD KW_WITH expression                 # CWith
    ;
// 带返回值的表达式, 能作为表达式的开头

op_compare:   OP_LT | OP_LEQ | OP_GT | OP_GEQ | OP_EQ | OP_NE | OP_EEE | OP_NEE;
op_pattern:   OP_AND | OP_OR;
infix_arrows: OP_ARROW | OP_ARROW2;
op_multiple:  OP_MUL | OP_DIV | OP_DIV | OP_DIV_REM;
op_plus:      OP_ADD | OP_SUB;
op_logic:     LOGIC_OR | LOGIC_AND | LOGIC_XOR | LOGIC_NOR | LOGIC_NAND | LOGIC_XAND;
op_pipeline:  OP_LL | OP_LLE | OP_LLL | OP_GG | OP_GGE | OP_GGG;
op_assign
    : OP_ASSIGN
    | OP_ADD_ASSIGN
    | OP_SUB_ASSIGN
    | OP_MUL_ASSIGN
    | OP_DIV_ASSIGN
    | OP_MAY_ASSIGN
    ;
infix_is: KW_IS | KW_IS KW_NOT | OP_IS | OP_IS_NOT | OP_CONTINUES;
infix_in: KW_IN | KW_NOT KW_IN | OP_IN | OP_NOT_IN;
// ===========================================================================
define_generic
    : GENERIC_L GENERIC_R
    | GENERIC_L generic_item (COMMA generic_item)* COMMA? GENERIC_R
    | OP_PROPORTION? OP_LT OP_GT
    | OP_PROPORTION? OP_LT generic_item (COMMA generic_item)* COMMA? OP_GT
    ;
generic_item: (identifier COLON)? type_expression;
generic_call
    : OP_PROPORTION OP_LT OP_GT
    | OP_PROPORTION OP_LT generic_pair (COMMA generic_pair)* COMMA? OP_GT
    | GENERIC_L GENERIC_R
    | GENERIC_L generic_pair (COMMA generic_pair)* COMMA? GENERIC_R
    ;
generic_call_in_type
    : OP_PROPORTION? OP_LT generic_pair (COMMA generic_pair)* OP_GT
    | GENERIC_L generic_pair (COMMA generic_pair)* GENERIC_R
    ;
generic_pair: (identifier COLON)? type_expression;
define_label: OP_LABEL identifier;
// ===========================================================================
offset_call: OP_PROPORTION BRACKET_L expression BRACKET_R | OFFSET_L expression OFFSET_R;
// ===========================================================================
template_call
    : annotation* modifiers KW_TEMPLATE template_block
    | annotation* modifiers KW_TEMPLATE identifier (COMMA identifier)* COMMA? template_block
    ;
template_block:      BRACE_L (template_statements | eos_free)* BRACE_R;
template_statements: KW_WHERE where_block;
where_block:         BRACE_L where_bound* BRACE_R;
where_bound:         identifier COLON type_expression | eos_free;
// ===========================================================================
macro_call
    : OP_AT annotation_call_item class_block?
    | OP_AT BRACKET_L annotation_call_item (COMMA annotation_call_item)* BRACKET_R class_block?
    ;
annotation
    : OP_HASH annotation_call_item class_block?
    | OP_HASH BRACKET_L annotation_call_item (COMMA annotation_call_item)* BRACKET_R class_block?
    ;
annotation_call_item: namepath tuple_call_body? class_block?;
// ===========================================================================
try_statement: annotation* KW_TRY type_expression? function_block;
match_statement
    : annotation* (KW_MATCH | KW_CATCH) (identifier OP_BIND)? inline_expression match_block
    ;
// ===========================================================================
match_call:  OP_THROW? DOT (KW_MATCH | KW_CATCH) (KW_AS identifier type_hint?)? match_block;
match_block: BRACE_L (match_terms | eos_free)* BRACE_R;
match_terms
    : annotation* KW_WITH identifier                                                   # MatchWith
    | annotation* KW_WITH BRACKET_L (identifier (COMMA identifier)* COMMA?)? BRACKET_R # MatchWithMany
    | annotation* KW_TYPE type_expression (KW_IF inline_expression)? match_case_block  # MatchType
    | annotation* KW_WHEN inline_expression match_case_block                           # MatchWhen
    | annotation* KW_ELSE match_case_block                                             # MatchElse
    | annotation* KW_CASE case_pattern (KW_IF inline_expression)? match_case_block     # MatchCase
    ;
match_case_block: COLON expression*;
case_pattern
    : case_pattern (OP_OR | OP_ADD) case_pattern # CaseOR
    | case_pattern (OP_UNTIL) case_pattern       # CaseUntil
    | case_pattern_item                          # CaseAtom
    ;
case_pattern_item
    : case_pattern_tuple
    | bind = identifier OP_BIND case_pattern_item
    | modified_identifier COLON (bind = identifier OP_BIND)? case_pattern_item
    | modified_identifier
    | namepath
    | number_literal
    | string_literal
    | SPECIAL
    ;
case_pattern_tuple
    : namepath? PARENTHESES_L PARENTHESES_R
    | namepath? BRACKET_L BRACKET_R
    | namepath? BRACE_L BRACE_R
    | PARENTHESES_L case_pattern_item COMMA PARENTHESES_R
    | PARENTHESES_L case_pattern_item (COMMA case_pattern_item)+ COMMA? PARENTHESES_R
    | namepath PARENTHESES_L case_pattern_item (COMMA case_pattern_item)* COMMA? PARENTHESES_R
    | namepath? BRACKET_L case_pattern_item (COMMA case_pattern_item)* COMMA? BRACKET_R
    | namepath? BRACE_L case_pattern_item (COMMA case_pattern_item)* COMMA? BRACE_R
    ;
// ===========================================================================
object_statement: KW_OBJECT define_generic? class_inherit? type_hint? class_block;
new_statement
    : KW_NEW modified_namepath generic_call_in_type? tuple_call_body? new_block
    | KW_NEW modified_namepath generic_call_in_type? tuple_call_body
    ;
new_body
    : tuple_call_body? new_block // 可选
    | tuple_call_body // 必选
    ;
new_block: BRACE_L (tuple_call_item | eos_free)* BRACE_R;
// ===========================================================================
tuple_literal
    : PARENTHESES_L PARENTHESES_R
    | PARENTHESES_L collection_pair (COMMA collection_pair)+ COMMA? PARENTHESES_R
    | PARENTHESES_L collection_pair COMMA PARENTHESES_R
    ;
collection_pair: (identifier COLON)? expression;
// ===========================================================================

// ===========================================================================
slice_call: OP_THROW? range_literal;
range_literal
    : BRACKET_L (range_axis (COMMA range_axis)* COMMA?)? BRACKET_R # Ordinal
    | OFFSET_L (range_axis (COMMA range_axis)* COMMA?)? OFFSET_R   # Offset
    ;
range_axis
    : COLON // [:]
    | OP_PROPORTION // [::]
    | range_start (COLON range_end)? (COLON range_step)?
    ;
range_start: inline_expression;
range_end:   inline_expression;
range_step:  inline_expression;
// ===========================================================================
modifiers:           identifier*;
modified_identifier: (mods += identifier)* id = identifier;
modified_namepath
    : (mods += identifier)* path += identifier (OP_PROPORTION path += identifier)*
    ;
// namepath
lambda_name:   LAMBDA_SLOT (identifier | number)?;
function_name: identifier (OP_PROPORTION identifier)* (DOT identifier)?;
namepath_free: identifier ((OP_PROPORTION | DOT) identifier)*;
namepath:      identifier (OP_PROPORTION identifier)*;
// identifier
identifier: UNICODE_ID | RAW_ID;
// number
number:         DECIMAL | INTEGER;
number_literal: number identifier?;
// string
string: STRING_SINGLE | STRING_DOUBLE;
string_literal
    : identifier? STRING_SINGLE
    | identifier? STRING_DOUBLE
    | identifier? STRING_BLOCK
    ;
