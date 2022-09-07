// inside error.rs
use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum SchnorrLibError {
    /// Not verified
    #[error("TSS Not Verified")]
    NotVerified
}

impl From<SchnorrLibError> for ProgramError {
    fn from(e: SchnorrLibError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
