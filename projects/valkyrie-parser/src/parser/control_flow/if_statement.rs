use super::*;

impl IfStatement {
    pub fn visit(&self, parser: &mut ValkyrieParser) -> ValkyrieResult<ValkyrieASTNode> {
        let condition = self.condition.visit(parser)?;
        // let mut this = If::new(condition);
        // for statement in &self.body.statements {
        //     statement.visit(parser, this.mut_body())?
        // }
        // Ok(this.to_node(parser.file, &self.position))
        todo!()
    }
}