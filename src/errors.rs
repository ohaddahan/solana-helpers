use solana_program::program_error::ProgramError;
use thiserror::Error;

/// Custom error types for the Solana helpers library.
#[derive(Error, Debug, Clone)]
pub enum Errors {
    /// Wraps a Solana program error with additional context.
    #[error("Program Error: {0}")]
    ProgramError(String),
    /// Indicates a numerical overflow occurred during calculations.
    #[error("Numerical Overflow")]
    NumericalOverflow,
    /// Indicates an owner mismatch validation error.
    #[error("Owner Mismatch")]
    OwnerMismatch,
    /// Indicates a PDA address doesn't match the expected derived address.
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
