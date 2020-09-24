use crate::ThisParser;
use valkyrie_ast::{GuardStatement, ProgramRoot, StatementBody, StatementNode};
use valkyrie_error::{SyntaxError, ValkyrieError};

trait Validation {
    fn validate(&mut self, errors: &mut Vec<SyntaxError>) -> Result<(), ValkyrieError>;
}

impl Validation for ProgramRoot {
    fn validate(&mut self, errors: &mut Vec<SyntaxError>) -> Result<(), ValkyrieError> {
        for statement in self.statements.iter_mut() {
            statement.validate(errors)?;
        }
        Ok(())
    }
}

impl Validation for StatementNode {
    fn validate(&mut self, errors: &mut Vec<SyntaxError>) -> Result<(), ValkyrieError> {
        match &mut self.r#type {
            StatementBody::Nothing => {
                todo!()
            }
            StatementBody::Document(_) => {
                todo!()
            }
            StatementBody::Annotation(_) => {
                todo!()
            }
            StatementBody::Namespace(_) => {
                todo!()
            }
            StatementBody::Import(_) => {
                todo!()
            }
            StatementBody::Class(_) => {
                todo!()
            }
            StatementBody::ClassField(_) => {
                todo!()
            }
            StatementBody::ClassMethod(_) => {
                todo!()
            }
            StatementBody::Union(_) => {
                todo!()
            }
            StatementBody::UnionField(_) => {
                todo!()
            }
            StatementBody::Enumerate(_) => {
                todo!()
            }
            StatementBody::EnumerateField(_) => {
                todo!()
            }
            StatementBody::Flags(_) => {
                todo!()
            }
            StatementBody::Tagged(_) => {
                todo!()
            }
            StatementBody::Variant(_) => {
                todo!()
            }
            StatementBody::Function(_) => {
                todo!()
            }
            StatementBody::While(_) => {
                todo!()
            }
            StatementBody::For(_) => {
                todo!()
            }
            StatementBody::LetBind(_) => {
                todo!()
            }
            StatementBody::Guard(e) => {
                e.validate(errors)?;
            }
            StatementBody::Control(_) => {
                todo!()
            }
            StatementBody::Expression(_) => {
                todo!()
            }
        }
        Ok(())
    }
}

impl Validation for GuardStatement {
    fn validate(&mut self, errors: &mut Vec<SyntaxError>) -> Result<(), ValkyrieError> {
        const MSG: &str = "Guard statement must a explicit control statement as the last statement";
        match self.last() {
            Some(StatementBody::Control(_)) => {}
            Some(other) => errors.push(SyntaxError::new(MSG).with_span(&other.get_range())),
            None => errors.push(SyntaxError::new(MSG).with_span(&self.body.span)),
        }
        Ok(())
    }
}
