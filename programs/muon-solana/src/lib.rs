use anchor_lang::prelude::*;

mod instructions;
mod state;
mod errors;
mod types;

declare_id!("BisbEpajuY8aT3GQ46bRhn2Xqmobqb41AQYNtkTqWsD7");

use crate::{
    instructions::*,
    types::SchnorrSign
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

    pub fn verify_signature(ctx: Context<VerifySignature>, req_id: [u8; 36], hash: [u8; 32], sign: SchnorrSign) -> Result<()> {
        msg!("TODO.");
        Ok(())
    }
}
