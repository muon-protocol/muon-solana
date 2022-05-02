use anchor_lang::prelude::*;

#[error_code]
pub enum MuonErrors {
    #[msg("Invalid Instruction")]
    InvalidInstruction,
    #[msg("Admin Already Initialized")]
    AdminAlreadyInitialized,
    #[msg("Invalid Admin Storage Account")]
    InvalidAdminStorage,
    #[msg("Invalid Admin Account")]
    InvalidAdminAccount,
    #[msg("Not Rent Exempt")]
    NotRentExempt,
    #[msg("admin Restricted")]
    AdminRestricted,
    #[msg("Missing Admin Signature")]
    MissingAdminSignature,
    #[msg("Invalid Group Account Owner")]
    InvalidGroupAccountOwner,
    #[msg("Large Public key X")]
    LargePubkeyX,
    #[msg("No Zero Inputs Allowed")]
    ZeroSignatureData,
    #[msg("Not Verified")]
    NotVerified,
}
