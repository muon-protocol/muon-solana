use anchor_lang::prelude::*;

mod instructions;
mod state;
mod errors;

declare_id!("HRYvN8EuFdi3N8BbyPhs3uHLSVr2RWNES4tR42Cs4xEv");

use crate::{
    instructions::*,
    state::*
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
        let admin = &mut ctx.accounts.admin;
        admin_info.admin = new_admin;
        Ok(())
    }
}
