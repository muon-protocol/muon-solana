use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError
};
use primitive_types::{ U256 as u256};
use muonv02::{
    types::U256Wrap,
    types::MuonRequestId,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Instruction {
    Verify {
        req_id: MuonRequestId,
        msg: String,
        signature_s: U256Wrap,
        owner: U256Wrap,
        nonce: U256Wrap
    }
}


