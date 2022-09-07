use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    pubkey::Pubkey
};
use crate::{
    types::U256Wrap
};


/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GroupInfo {
    pub is_valid: bool,
    pub eth_address: U256Wrap,
    pub pubkey_x: U256Wrap,
    pub pubkey_y_parity: u8
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
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
