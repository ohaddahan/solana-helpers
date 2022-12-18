use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum Errors {
    /// Invalid instruction
    #[error("Numerical Overflow")]
    NumericalOverflow,
}

impl From<Errors> for ProgramError {
    fn from(e: Errors) -> Self {
        ProgramError::Custom(e as u32)
    }
}