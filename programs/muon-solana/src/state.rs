use anchor_lang::{
    prelude::*,
    solana_program::{pubkey::Pubkey}
};
use crate::{types::u256};

#[account]
#[derive(Default)]
pub struct AdminInfo {
    pub admin: Pubkey,
    pub last: u32
}

impl AdminInfo {
    pub fn is_initialized(&self) -> bool {
        !self.admin.eq(&Pubkey::default())
    }

    pub fn space() -> usize {
        // discriminator + admin pubkey + last
        8 + 32 + 4
    }
}

#[account]
#[derive(Default)]
pub struct GroupInfo {
    pub is_valid: bool,
    pub eth_address: u256,
    pub pubkey_x: u256,
    pub pubkey_y_parity: u8
}

impl GroupInfo {
    pub fn is_initialized(&self) -> bool {
        !self.eth_address.0.is_zero()
    }

    pub fn space() -> usize {
        // discriminator + boolean + u256 + u256 + u8
        8 + 1 + 32 + 32 + 1
    }
}
