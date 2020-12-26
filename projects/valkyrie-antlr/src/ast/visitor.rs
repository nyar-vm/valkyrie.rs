use super::*;
use crate::extractors::Extractor;
use antlr_rust::{tree::Tree, TidExt};
use valkyrie_ast::{NamePathNode, NamespaceDeclaration};

impl ParseTreeVisitorCompat<'_> for ValkyrieProgramParser {
    type Node = ValkyrieAntlrParserContextType;
    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        unreachable!()
    }
    fn visit(&mut self, node: &<Self::Node as ParserNodeType<'_>>::Type) -> Self::Return {
        node.accept_dyn(self);
    }
}

/// Convert weakly typed ast to strongly typed ast
impl ValkyrieAntlrVisitor<'_> for ValkyrieProgramParser {
    fn visit_program(&mut self, ctx: &ProgramContext<'_>) {
        for node in ctx.top_statement_all() {
            self.visit_top_statement(&node);
        }
    }

    fn visit_top_statement(&mut self, ctx: &Top_statementContext<'_>) {
        if let Some(s) = NamespaceDeclaration::take(ctx.define_namespace()) {
            self.statements.push(StatementNode { r#type: StatementType::Namespace(Box::new(s)), end_semicolon: false });
            return;
        }
        if let Some(s) = ClassDeclaration::take(ctx.define_class()) {
            self.statements.push(StatementNode { r#type: StatementType::Class(Box::new(s)), end_semicolon: false });
            return;
        }
    }
}
