use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{ next_account_info, AccountInfo },
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::{
        Pubkey,
        ParsePubkeyError
    },
//    instruction::{
//        Instruction,
//        AccountMeta
//    },
    program::{
        invoke
    }
};
use sha3::{Digest, Keccak256};
use primitive_types::{ U256 as u256, U512 as u512 };

use crate::{
    instructions::Instruction,
    errors::SchnorrLibError
};
use muonv04::{
    instructions::MuonInstruction,
    types::{U256Wrap, MuonRequestId, GroupPubKey},
};


pub struct Processor;

impl Processor {
    //TODO: save group public key
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("sample program start.");

//        let instruction = VerifyInstruction::unpack(instruction_data)?;

        let instruction = Instruction::try_from_slice(instruction_data)
            .map_err(|e| ProgramError::InvalidInstructionData)?;

        match instruction {
            Instruction::Verify{req_id, msg, signature_s, owner, nonce} => {
                Self::process_verify(_program_id, accounts, req_id, msg, signature_s, owner, nonce)
            }
        }
    }

    pub fn process_verify(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        req_id: MuonRequestId,
        msg: String,
        signature_s: U256Wrap,
        owner: U256Wrap,
        nonce: U256Wrap,
    ) -> ProgramResult {

//        msg!(
//            "req_id: {:x}, msg: {}, signature_s: {:02x}, owner: {:02x}, nonce: {:02x}",
//            req_id, msg, signature_s, owner, nonce
//        );

        // Iterating accounts is safer then indexing
        let accounts_iter = &mut accounts.iter();

        // Get the account to store admin info
        let group_info_storage = next_account_info(accounts_iter)?;
//        msg!("group_info: {:x?}.", group_info_storage.key);

        let caller = next_account_info(accounts_iter)?;
//        msg!("caller: {:x?}.", caller.key);

        let muon = next_account_info(accounts_iter)?;
//        msg!("muon: {:x?}.", muon.key);

        let msg_hash = Self::hash_parameters(msg);
//        msg!("msg_hash: {:x}.", msg_hash);

        //let parity: U256Wrap = U256Wrap{0:};
        let ix = MuonInstruction::verify(
            // Address of account that stores signer data.
            *group_info_storage.key,
            // muon request ID.
            &req_id.0,
            // message hash
            msg_hash,
            // s part of signature
            signature_s.0,
            // ethereum address of signer as u256.
            owner.0,
            // ethereum address of signature nonce.
            nonce.0,

            //TODO: FixMe
            u256([0,0,0,0]), // pub_key_x
            0, //pub_key_parity
        );

        invoke(
            &ix,
            &[
                group_info_storage.clone(),
                caller.clone(),
                muon.clone()
            ]
        )?;

        Ok(())
    }

    fn hash_parameters(msg: String) -> u256 {
        let mut hasher = Keccak256::new();
        hasher.update(msg);
        let result = hasher.finalize();
        u256::from(&result[..])
    }
}
