use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeAdmin<'info> {
    #[account(
        init, payer = admin_account, space=8 + 8,
        seeds = [b"admin"], bump = bump
    )]
    pub admin_info: Account<'info, AdminInfo>,
    pub admin_account: Signer<'info>,
    pub rent_program: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>
}

//#[derive(Accounts)]
//pub struct TweetPlatform<'info> {
//    #[account(init, payer = user, space = 9000 )]
//    pub tweet: Account<'info, Tweet>,
//    #[account(mut)]
//    pub user: Signer<'info>,
//    pub system_program: Program<'info, System>,
//}
//
//#[derive(Accounts)]
//pub struct WriteTweet<'info> {
//    #[account(mut)]
//    pub tweet: Account<'info, Tweet>
//}
//
//#[derive(Accounts)]
//pub struct LikeTweet<'info> {
//    #[account(mut)]
//    pub tweet: Account<'info, Tweet>
//}
