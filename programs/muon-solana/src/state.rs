use anchor_lang::{
    prelude::*,
    solana_program::{pubkey::Pubkey}
};
use crate::{types::U256Wrap};

#[account]
#[derive(Default)]
pub struct GroupInfo {
    pub is_valid: bool,
    pub eth_address: U256Wrap,
    pub pubkey_x: U256Wrap,
    pub pubkey_y_parity: u8
}

#[account]
#[derive(Default)]
pub struct AdminInfo {
    pub admin: Pubkey,
    pub last: u32
}

impl GroupInfo {
    pub fn is_initialized(&self) -> bool {
        !self.eth_address.0.is_zero()
    }
}

impl AdminInfo {
    pub fn is_initialized(&self) -> bool {
        !self.admin.eq(&Pubkey::default())
    }
}
