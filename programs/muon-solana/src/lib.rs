use anchor_lang::prelude::*;

mod instructions;
mod state;
mod errors;
mod types;

declare_id!("BisbEpajuY8aT3GQ46bRhn2Xqmobqb41AQYNtkTqWsD7");

use crate::{
    instructions::*,
    types::u256
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

    pub fn add_group(ctx: Context<AddGroup>, eth_address: u256, pubkey_x: u256, pubkey_y_parity: u8) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        storage.is_valid = true;
        storage.eth_address = eth_address;
        storage.pubkey_x = pubkey_x;
        storage.pubkey_y_parity = pubkey_y_parity;
        Ok(())
    }
}
