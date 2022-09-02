use borsh::{BorshDeserialize, BorshSerialize};
use primitive_types::{ U256 as u256};
use solana_program::{
    pubkey::{
        Pubkey
    },
    instruction::{
        Instruction,
        AccountMeta
    },
};

use crate::{
    types::{U256Wrap, SchnorrSign, GroupPubKey, MuonRequestId}
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
//#[derive(Serialize, Deserialize, Debug)]
pub enum MuonInstruction {
    /// Accounts expected
    ///
    /// 0. `[writable]` admin info storage account
    /// 1. `[]` admin account
    /// 2. `[]` the rent sysvar
    InitializeAdmin,

    /// Accounts expected
    ///
    /// 0. `[writable]` admin info storage account
    /// 1. `[signer]` old admin account
    /// 2. `[]` new admin account
    TransferAdmin,

    /// Accounts expected
    ///
    /// 0. `[writable]` group info storage account
    /// 1. `[]` admin info storage account
    /// 2. `[]` admin account
    /// 3. `[]` the rent sysvar
    
    // AddGroup {
    //     eth_address: U256Wrap,
    //     pubkey_x: U256Wrap,
    //     pubkey_y_parity: u8,
    // },

    /// Accounts expected
    ///
    /// 0. `[]` group info storage account
    VerifySignature {
        req_id: MuonRequestId,
        hash: U256Wrap,
        sign: SchnorrSign,
        pub_key: GroupPubKey
    },
    // TODO
    // RemoveGroup{eth_address: u256}
}

impl MuonInstruction {

    pub fn verify(
        // Address of account that stores signer data.
        group_info_storage: Pubkey,
        // muon request ID.
        req_id: &[u8; 36],
        // message hash
        hash: u256,
        // s part of signature
        signature_s: u256,
        // ethereum address of signer as u256.
        signer: u256,
        // ethereum address of signature nonce.
        nonce: u256,
        pub_key_x: u256,
        pub_key_parity: u8
    ) -> Instruction {
        Instruction::new_with_borsh(
            // program_id
            crate::id(),
            // instruction
            &Self::VerifySignature {
                req_id: MuonRequestId(*req_id),
                hash: U256Wrap(hash),
                sign: SchnorrSign {
                    signature: U256Wrap(signature_s),
                    address: U256Wrap(signer),
                    nonce: U256Wrap(nonce)
                },
                pub_key: GroupPubKey {
                    x: U256Wrap(pub_key_x),
                    parity: pub_key_parity
                }
            },
            //accounts
            vec![
                AccountMeta::new_readonly(group_info_storage, false)
            ]
        )
    }
}
