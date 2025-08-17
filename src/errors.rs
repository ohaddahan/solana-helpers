use solana_program::entrypoint::ProgramResult;
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
    #[error("Can't convert ProgramResult")]
    CantConvertProgramResult,
}

impl From<ProgramResult> for Errors {
    fn from(value: ProgramResult) -> Self {
        match value.err() {
            Some(err) => Self::from(err),
            None => Self::CantConvertProgramResult,
        }
    }
}

impl From<ProgramError> for Errors {
    fn from(value: ProgramError) -> Self {
        Self::ProgramError(value.to_string())
    }
}
