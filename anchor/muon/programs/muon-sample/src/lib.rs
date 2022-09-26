use anchor_lang::prelude::*;
use muon::{
    types::GroupPubKey
};
use muon::{self};

declare_id!("EbqAz7dRNg57aMVsgPxt294nTpU54FDtsrqwVKsRBTnj");

#[program]
pub mod muon_sample {
    use super::*;

    pub fn initialize(
    	ctx: Context<Initialize>, 
    	muon_pub_key: GroupPubKey) -> Result<()> {

    	let muon_app_account = &mut ctx.accounts.muon_app_info;
    	muon_app_account.group_key = muon_pub_key;

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
        space = 8 + 256 + 8, seeds = [b"muon-app-info"], bump
    )]
    pub muon_app_info: Account<'info, MuonInfo>
}
