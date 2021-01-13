use crate::ThisParser;
use valkyrie_ast::{GuardStatement, ProgramRoot, StatementNode};
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
        self.r#type.validate(errors)
    }
}

impl Validation for StatementNode {
    fn validate(&mut self, errors: &mut Vec<SyntaxError>) -> Result<(), ValkyrieError> {
        match self {
            Self::Nothing => {
                todo!()
            }
            Self::Document(_) => {
                todo!()
            }
            Self::Annotation(_) => {
                todo!()
            }
            Self::Namespace(_) => {
                todo!()
            }
            Self::Import(_) => {
                todo!()
            }
            Self::Class(_) => {
                todo!()
            }
            Self::ClassField(_) => {
                todo!()
            }
            Self::ClassMethod(_) => {
                todo!()
            }
            Self::Union(_) => {
                todo!()
            }
            Self::UnionField(_) => {
                todo!()
            }
            Self::Enumerate(_) => {
                todo!()
            }
            Self::EnumerateField(_) => {
                todo!()
            }
            Self::Flags(_) => {
                todo!()
            }
            Self::Tagged(_) => {
                todo!()
            }
            Self::Variant(_) => {
                todo!()
            }
            Self::Function(_) => {
                todo!()
            }
            Self::While(_) => {
                todo!()
            }
            Self::For(_) => {
                todo!()
            }
            Self::LetBind(_) => {
                todo!()
            }
            Self::Guard(e) => e.validate(errors),
            Self::Control(_) => {
                todo!()
            }
            Self::Expression(_) => {
                todo!()
            }
            Self::GuardLet(_) => {
                todo!()
            }
        }
    }
}

impl Validation for GuardStatement {
    fn validate(&mut self, errors: &mut Vec<SyntaxError>) -> Result<(), ValkyrieError> {
        const MSG: &str = "Guard statement must a explicit control statement as the last statement";
        match self.last() {
            Some(StatementNode::Control(_)) => {}
            Some(other) => errors.push(SyntaxError::new(MSG).with_span(&other.get_range())),
            None => errors.push(SyntaxError::new(MSG).with_span(&self.main_body.get_range())),
        }
        Ok(())
    }
}
