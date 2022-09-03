use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    pubkey::Pubkey
};
use crate::{
    types::U256Wrap
};

//const Q: u256 = u256::from("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141");
//const HALF_Q: u256 = (Q >> 1) + 1;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
//#[derive(Serialize, Deserialize, Debug)]
pub struct GroupInfo {
    pub is_valid: bool,
    pub eth_address: U256Wrap,
    pub pubkey_x: U256Wrap,
    pub pubkey_y_parity: u8
}

pub struct GroupPubKey {
    pub pubkey_x: U256Wrap,
    pub pubkey_y_parity: u8
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
//#[derive(Serialize, Deserialize, Debug)]
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
