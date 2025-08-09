use crate::error::CodeGenerationError;

pub mod types;

pub trait GenerateCode<TY, TARGET>
where
    TARGET:,
{
    fn generate_code(&self) -> Result<(), CodeGenerationError>;
}
