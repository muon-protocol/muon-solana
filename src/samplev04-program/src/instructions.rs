use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError
};
use primitive_types::{ U256 as u256};
use muonv04::{
    types::U256Wrap,
    types::MuonRequestId,
    types::GroupPubKey
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Instruction {
    Verify {
        req_id: MuonRequestId,
        msg: String,
        signature_s: U256Wrap,
        owner: U256Wrap,
        nonce: U256Wrap
    },

    InitializeAdmin,

    TransferAdmin,

    UpdateGroupPubKey {
        pubkey_x: U256Wrap,
        pubkey_y_parity: u8,
    }
}
