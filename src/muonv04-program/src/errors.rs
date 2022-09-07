// inside error.rs
use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum MuonError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Admin Already Initialized")]
    AdminAlreadyInitialized,
    #[error("Invalid Admin Storage Account")]
    InvalidAdminStorage,
    #[error("Invalid Admin Account")]
    InvalidAdminAccount,
    #[error("Not Rent Exempt")]
    NotRentExempt,
    #[error("Admin Restricted")]
    AdminRestricted,
    #[error("Missing Admin Signature")]
    MissingAdminSignature,
    #[error("Invalid Group Account Owner")]
    InvalidGroupAccountOwner,
    #[error("Large Public key X")]
    LargePubkeyX,
    #[error("No Zero Inputs Allowed")]
    ZeroSignatureData,
    #[error("TSS Not Verified")]
    NotVerified,
}

impl From<MuonError> for ProgramError {
    fn from(e: MuonError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
