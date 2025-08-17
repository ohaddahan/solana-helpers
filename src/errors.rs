use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Errors {
    #[error("Program Error: {0}")]
    ProgramError(String),
    /// Invalid instruction
    #[error("Numerical Overflow")]
    NumericalOverflow,
    #[error("Owner Mismatch")]
    OwnerMismatch,
    #[error("Wrong Pda Address")]
    WrongPdaAddress,
}

// impl From<Errors> for ProgramError {
//     fn from(e: Errors) -> Self {
//         ProgramError::Custom(e as u32)
//     }
// }

impl From<ProgramError> for Errors {
    fn from(value: ProgramError) -> Self {
        Self::ProgramError(value.to_string())
    }
}
