use valkyrie_types::ValkyrieError;

pub trait ThisValidator
where
    Self: Sized,
{
    fn validate(&self, errors: &mut Vec<ValkyrieError>) -> Result<(), ValkyrieError>;
}
