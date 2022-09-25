use anchor_lang::prelude::*;
use muon::{
    types::GroupPubKey
};
use muon::{self};

declare_id!("HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L");

#[program]
pub mod muon_sample {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    // pub fn verify_tss(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // }
}

#[account]
pub struct MuonInfo {
	pub group_key: GroupPubKey
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        init,
        payer = user,
        space = 8 + 256 + 8, seeds = [b"muon-app-info", system_program.key().as_ref()], bump
    )]
    pub muon_app_info: Account<'info, MuonInfo>
}
