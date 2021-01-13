mod extractors;
mod visitor;
use crate::{
    antlr::{
        valkyrieantlrlexer::ValkyrieAntlrLexer,
        valkyrieantlrparser::{self, *},
        valkyrieantlrvisitor::ValkyrieAntlrVisitor,
    },
    traits::Extractor,
};
use antlr_rust::{
    common_token_stream::CommonTokenStream,
    errors::ANTLRError,
    parser::ParserNodeType,
    parser_rule_context::ParserRuleContext,
    tree::{ParseTree, ParseTreeVisitorCompat, TerminalNode, Tree, VisitChildren, VisitableDyn},
    CoerceTo, InputStream, TidExt,
};
use std::{ops::Range, rc::Rc};
use valkyrie_ast::{
    ClassDeclaration, ClassFieldDeclaration, ClassKind, ExpressionNode, ExpressionType, FlagsDeclaration, ForLoop,
    IdentifierNode, InfixNode, LetPattern, LogicMatrix, ModifiersNode, NamePathNode, NamespaceDeclaration, NamespaceKind,
    NumberLiteralNode, OperatorNode, PostfixNode, PrefixNode, ProgramRoot, StatementNode, UnionDeclaration, ValkyrieOperator,
};

#[derive(Clone, Debug, Default)]
pub struct ValkyrieProgramParser {
    statements: Vec<StatementNode>,
}

impl ValkyrieProgramParser {
    pub fn parse(input: &str) -> Result<ProgramRoot, ANTLRError> {
        let codepoints = input.chars().map(|x| x as u32).collect::<Vec<_>>();
        let input = InputStream::new(&*codepoints);
        let lexer = ValkyrieAntlrLexer::new(input);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = ValkyrieAntlrParser::new(token_source);
        let root = parser.program()?;
        let mut state = ValkyrieProgramParser::default();
        state.visit(&*root);
        Ok(ProgramRoot { statements: state.statements })
    }
}
