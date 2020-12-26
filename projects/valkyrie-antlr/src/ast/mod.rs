mod extractors;
mod visitor;
use crate::{
    antlr::{
        valkyrieantlrlexer::ValkyrieAntlrLexer,
        valkyrieantlrlistener::ValkyrieAntlrListener,
        valkyrieantlrparser::{self, *},
        valkyrieantlrvisitor::ValkyrieAntlrVisitor,
    },
    extractors::Extractor,
};
use antlr_rust::{
    common_token_stream::CommonTokenStream,
    errors::ANTLRError,
    parser::ParserNodeType,
    parser_rule_context::ParserRuleContext,
    token::Token,
    tree::{ParseTree, ParseTreeListener, ParseTreeVisitorCompat, TerminalNode, VisitChildren, VisitableDyn},
    CoerceTo, InputStream,
};
use std::{ops::Range, rc::Rc};
use valkyrie_ast::{
    ClassDeclaration, ClassKind, IdentifierNode, NamePathNode, NamespaceDeclaration, NamespaceKind, ProgramRoot, StatementNode,
    StatementType,
};

#[derive(Clone, Debug, Default)]
pub struct ValkyrieProgramParser {
    statements: Vec<StatementNode>,
}

struct Listener3;

impl ValkyrieAntlrListener<'_> for Listener3 {}
impl<'input> ParseTreeListener<'input, ValkyrieAntlrParserContextType> for Listener3 {
    fn visit_terminal(&mut self, node: &TerminalNode<'input, ValkyrieAntlrParserContextType>) {
        println!("terminal node {}", node.symbol.get_text());
    }

    fn enter_every_rule(&mut self, ctx: &<ValkyrieAntlrParserContextType as ParserNodeType>::Type) {
        println!("rule entered {}", ruleNames.get(ctx.get_rule_index()).unwrap_or(&"error"))
    }
    fn exit_every_rule(&mut self, ctx: &<ValkyrieAntlrParserContextType as ParserNodeType>::Type) {
        println!("rule exited {}", ruleNames.get(ctx.get_rule_index()).unwrap_or(&"error"))
    }
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
    // pub fn walk(input: &str) {
    //     let mut _lexer = ValkyrieAntlrLexer::new(InputStream::new(input.into()));
    //     let token_source = CommonTokenStream::new(_lexer);
    //     let mut parser = ValkyrieAntlrParser::new(token_source);
    //     let result = parser.program().expect("expected to parse successfully");
    //     let listener = ValkyrieAntlrTreeWalker::walk(Box::new(Listener3), &*result);
    //     // assert_eq!(&listener, "terminal node x\nterminal node y\nterminal node z\n");
    // }
}
