use anchor_lang::prelude::*;
use crate::{
    state::*,
    errors::MuonErrors,
    types::SchnorrSign
};

#[derive(Accounts)]
pub struct InitializeAdmin<'info> {
    #[account(
        init,
        payer = admin,
        space = AdminInfo::space(),
        seeds = [b"admin"],
        bump
    )]
    pub admin_info: Account<'info, AdminInfo>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub rent_program: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct TransferAdmin<'info> {
    #[account(
        mut,
        seeds = [b"admin"],
        bump
    )]
    pub admin_info: Account<'info, AdminInfo>,
    #[account(
        mut,
        constraint = admin_info.admin == admin.key() @ MuonErrors::AdminRestricted
    )]
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(eth_address: [u8; 32], pubkey_x: [u8; 32], pubkey_y_parity: u8)]
pub struct AddGroup<'info> {
    #[account(
        init,
        payer = admin,
        space = GroupInfo::space(),
        seeds = [
            b"group-info".as_ref(),
            &eth_address
        ],
        bump
    )]
    pub storage: Account<'info, GroupInfo>,
    pub admin_info: Account<'info, AdminInfo>,
    #[account(
        mut,
        constraint = admin_info.admin == admin.key() @ MuonErrors::AdminRestricted
    )]
    pub admin: Signer<'info>,
    pub rent_program: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(req_id: [u8; 36], hash: [u8; 32], sign: SchnorrSign)]
pub struct VerifySignature<'info> {
    #[account(
        constraint = group_info.is_valid == true @ MuonErrors::NotVerified
    )]
    pub group_info: Account<'info, GroupInfo>
}
