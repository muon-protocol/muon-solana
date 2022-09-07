use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{ next_account_info, AccountInfo },
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::{
        Pubkey
    },

    program::{
        invoke
    },
    sysvar::{rent::Rent, Sysvar},
};
use sha3::{Digest, Keccak256};
use primitive_types::{ U256 as u256};

use crate::{
    instructions::Instruction
};
use muonv04::{
    instructions::MuonInstruction,
    types::{U256Wrap, MuonRequestId, MuonAppInfo},
    errors::MuonError,
    state::{AdminInfo}
};


pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {

        let instruction = Instruction::try_from_slice(instruction_data)
            .map_err(|e| ProgramError::InvalidInstructionData)?;

        match instruction {
            Instruction::Verify{req_id, msg, signature_s, nonce} => {
                Self::process_verify(program_id, accounts, req_id, msg, signature_s, nonce)
            }
            Instruction::InitializeAdmin => {
                Self::process_initialize_admin(program_id, accounts)
            }
            Instruction::TransferAdmin => {
                Self::process_transfer_admin(program_id, accounts)
            }
            Instruction::UpdateGroupPubKey { pubkey_x, pubkey_y_parity, muon_app_id } => {
                Self::process_update_group(program_id, accounts, pubkey_x,
                    pubkey_y_parity, muon_app_id)
            }
        }
    }

    pub fn process_verify(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        req_id: MuonRequestId,
        msg: String,
        signature_s: U256Wrap,
        nonce: U256Wrap,
    ) -> ProgramResult {
        // Iterating accounts is safer then indexing
        let accounts_iter = &mut accounts.iter();

        let group_info_storage = next_account_info(accounts_iter)?;

        let caller = next_account_info(accounts_iter)?;

        // TODO: validate Muon
        let muon = next_account_info(accounts_iter)?;

        let group_info = MuonAppInfo::try_from_slice(&group_info_storage.data.borrow())?;

        let msg_hash = Self::hash_parameters(
            msg,
            &req_id,
            &group_info.muon_app_id
        );

        let ix = MuonInstruction::verify(
            // Address of account that stores group info
            *group_info_storage.key,
            // muon request ID.
            &req_id.0,
            // message hash
            msg_hash,
            // s part of signature
            signature_s.0,
            // nonce
            nonce.0,
            // pub_key_x
            group_info.group_pub_key.x.0, 
            //pub_key_parity
            group_info.group_pub_key.parity,
        );

        // panics when the signature is not valid
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

    pub fn process_initialize_admin(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {

        // TODO: should we verify the signer?

        // Iterating accounts is safer then indexing
        let accounts_iter = &mut accounts.iter();

        // Get the account to store admin info
        let admin_info_storage = next_account_info(accounts_iter)?;

        Self::validate_admin_storage(program_id, admin_info_storage)?;

        // Increment and store the number of times the account has been greeted
        let mut admin_info = AdminInfo::try_from_slice(&admin_info_storage.data.borrow())?;

        let admin = next_account_info(accounts_iter)?;

        Self::is_rent_exempt(next_account_info(accounts_iter)?, admin_info_storage)?;

        if admin_info.is_initialized() {
            msg!("Already initialized.");
            return Err(MuonError::AdminAlreadyInitialized.into());
        }

        admin_info.admin = *admin.key;
        admin_info.serialize(&mut &mut admin_info_storage.data.borrow_mut()[..])?;

        msg!("InitializeAdmin Done.");

        Ok(())
    }

    pub fn process_transfer_admin(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        // Iterating accounts is safer then indexing
        let accounts_iter = &mut accounts.iter();

        // Get the account to store admin info
        let admin_info_storage = next_account_info(accounts_iter)?;

        Self::validate_admin_storage(program_id, admin_info_storage)?;

        let mut admin_info = AdminInfo::try_from_slice(&admin_info_storage.data.borrow())?;

        // The account must be owned by the program in order to modify its data
        if !admin_info.is_initialized() {
            msg!("AdminInfo account does not have the correct program id");
            return Err(MuonError::InvalidAdminStorage.into());
        }

        let old_admin = next_account_info(accounts_iter)?;
        let new_admin = next_account_info(accounts_iter)?;

        if admin_info.admin != *old_admin.key {
            msg!("old admin mismatched.");
            return Err(ProgramError::InvalidAccountData);
        }

        if !old_admin.is_signer {
            msg!("old admin is not signer.");
            return Err(ProgramError::MissingRequiredSignature);
        }

        msg!("TransferAdmin from {} to {}", *old_admin.key, *new_admin.key);
        admin_info.admin = *new_admin.key;
        admin_info.serialize(&mut &mut admin_info_storage.data.borrow_mut()[..])?;

        msg!("TransferAdmin Done.");

        Ok(())
    }

    pub fn process_update_group(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        pubkey_x: U256Wrap,
        pubkey_y_parity: u8,
        muon_app_id: U256Wrap
    ) -> ProgramResult {
        msg!("AddGroup x:0x{:x} y_parity: {}", &pubkey_x, pubkey_y_parity);

        // Iterating accounts is safer then indexing
        let accounts_iter = &mut accounts.iter();

        // Get the account to store admin info
        let group_info_storage = next_account_info(accounts_iter)?;

        // The account must be owned by the program in order to modify its data
        if group_info_storage.owner != program_id {
            msg!("GroupInfo account does not have the correct program id");
            return Err(MuonError::InvalidGroupAccountOwner.into());
        }

        let admin_storage = next_account_info(accounts_iter)?;

        Self::validate_admin_storage(program_id, admin_storage)?;

        let admin_info = AdminInfo::try_from_slice(&admin_storage.data.borrow())?;

        let admin = next_account_info(accounts_iter)?;

        Self::is_rent_exempt(next_account_info(accounts_iter)?, group_info_storage)?;

        if admin_info.admin != *admin.key {
            msg!("Admin restricted.");
            return Err(MuonError::AdminRestricted.into());
        }

        if !admin.is_signer {
            msg!("Admin is not signer.");
            return Err(MuonError::MissingAdminSignature.into());
        }

        //TODO: rename group_pub_key to muon_app_info
        let mut group_pub_key = MuonAppInfo::try_from_slice(&group_info_storage.data.borrow())?;

        group_pub_key.group_pub_key.x = pubkey_x;
        group_pub_key.group_pub_key.parity = pubkey_y_parity;
        group_pub_key.muon_app_id = muon_app_id;

        group_pub_key.serialize(&mut &mut group_info_storage.data.borrow_mut()[..])?;

        msg!("AddGroup Done.");

        Ok(())
    }

    fn hash_parameters(
        msg: String, 
        req_id: &MuonRequestId,
        muon_app_id: &U256Wrap
    ) -> u256 {
        let mut hasher = Keccak256::new();

        let mut bytes: [u8; 32] = [0; 32];
        muon_app_id.0.to_big_endian(&mut bytes);

        hasher.update(&bytes);

        hasher.update(&req_id.0);

        hasher.update(msg);
        let result = hasher.finalize();

        u256::from(&result[..])
    }

    fn validate_admin_storage(program_id: &Pubkey, admin_info_storage: &AccountInfo) -> ProgramResult {
        // The account must be owned by the program in order to modify its data
        if admin_info_storage.owner != program_id {
            msg!("AdminInfo account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        let admin_storage_pubkey = Pubkey::create_with_seed(program_id, &"admin", program_id)?;
        if *admin_info_storage.key != admin_storage_pubkey {
            msg!("AdminInfo account address mismatch.");
            return Err(MuonError::InvalidAdminStorage.into());
        }

        Ok(())
    }

    fn is_rent_exempt(sysvar_account_info: &AccountInfo, account_info: &AccountInfo) -> ProgramResult {
        let rent = &Rent::from_account_info(sysvar_account_info)?;
        if !rent.is_exempt(account_info.lamports(), account_info.data_len()) {
            msg!("Account is not rent exempt.");
            return Err(MuonError::NotRentExempt.into());
        }

        Ok(())
    }
}
