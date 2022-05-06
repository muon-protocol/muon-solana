use anchor_lang::prelude::*;

mod instructions;
mod state;
mod errors;
mod types;
mod utils;

declare_id!("9jmDacsonqdLCn3a9NR3YiAXDnwXcduHvGMnip2jTjgd");

use crate::{
    instructions::*,
    types::{ SchnorrSign, MuonRequestId },
    utils::schnorr_verify,
};

#[program]
pub mod muon_solana {
    use super::*;

    pub fn initialize_admin(ctx: Context<InitializeAdmin>) -> Result<()> {
        let admin_info = &mut ctx.accounts.admin_info;
        admin_info.admin = *ctx.accounts.admin.key;
        Ok(())
    }

    pub fn transfer_admin(ctx: Context<TransferAdmin>, new_admin: Pubkey) -> Result<()> {
        let admin_info = &mut ctx.accounts.admin_info;
        admin_info.admin = new_admin;
        Ok(())
    }

    pub fn add_group(ctx: Context<AddGroup>, eth_address: [u8; 32], pubkey_x: [u8; 32], pubkey_y_parity: u8) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        storage.is_valid = true;
        storage.eth_address = eth_address;
        storage.pubkey_x = pubkey_x;
        storage.pubkey_y_parity = pubkey_y_parity;
        Ok(())
    }

    pub fn verify_signature(ctx: Context<VerifySignature>, req_id: MuonRequestId, hash: [u8; 32], sign: SchnorrSign) -> Result<()> {
        let group_info = &mut ctx.accounts.group_info;

        schnorr_verify(
            // [u8; 32] signer x
            group_info.pubkey_x,
            // [u8] signer y parity
            group_info.pubkey_y_parity,
            // [u8; 32] signature s
            sign.signature,
            // [u8; 32] msg hash
            hash,
            // [u8; 32] nonce address
            sign.nonce
        )?;

        msg!("req_id: [{:x}]", req_id);
        Ok(())
    }
}
