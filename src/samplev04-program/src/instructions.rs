use borsh::{BorshDeserialize, BorshSerialize};
use muonv04::{
    types::U256Wrap,
    types::MuonRequestId,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Instruction {
    InitializeAdmin,

    TransferAdmin,

    UpdateGroupPubKey {
        pubkey_x: U256Wrap,
        pubkey_y_parity: u8,

        muon_app_id: U256Wrap
    },

    Verify {
        req_id: MuonRequestId,
        msg: String,
        signature_s: U256Wrap,
        nonce: U256Wrap
    },
}
