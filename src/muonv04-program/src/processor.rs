use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::{ Pubkey },
    sysvar::{rent::Rent, Sysvar},
};
use crate::{
    types::{U256Wrap, SchnorrSign, GroupPubKey, MuonRequestId},
    instructions::MuonInstruction,
    state::{AdminInfo},
    errors::MuonError,
    utils::schnorr_verify
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // msg!("MuonV04: Processor");
        let instruction = MuonInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            MuonInstruction::InitializeAdmin => {
                Self::process_initialize_admin(program_id, accounts)
            }
            MuonInstruction::TransferAdmin => {
                Self::process_transfer_admin(program_id, accounts)
            }
            MuonInstruction::VerifySignature { req_id, hash, sign, pub_key} => {
                Self::process_verify_sign(program_id, accounts, req_id, hash, sign, pub_key)
            }
        }
    }

    pub fn process_initialize_admin(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        // TODO: is need to program KeyPair sign this transaction?

        // Iterating accounts is safer then indexing
        let accounts_iter = &mut accounts.iter();

        // Get the account to store admin info
        let admin_info_storage = next_account_info(accounts_iter)?;

        Self::validate_admin_storage(program_id, admin_info_storage)?;

        let mut admin_info = AdminInfo::try_from_slice(&admin_info_storage.data.borrow())?;

        let admin = next_account_info(accounts_iter)?;

        Self::is_rent_exempt(next_account_info(accounts_iter)?, admin_info_storage)?;

        if admin_info.is_initialized() {
            msg!("already initialized.");
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

        if !admin_info.is_initialized() {
            msg!("AdminInfo account does not have the correct program id");
            return Err(MuonError::InvalidAdminStorage.into());
        }

        let old_admin = next_account_info(accounts_iter)?;
        let new_admin = next_account_info(accounts_iter)?;

        if admin_info.admin != *old_admin.key {
            msg!("Old admin mismatch.");
            return Err(ProgramError::InvalidAccountData);
        }

        if !old_admin.is_signer {
            msg!("Old admin is not signer.");
            return Err(ProgramError::MissingRequiredSignature);
        }

        msg!("TransferAdmin from {} to {}", *old_admin.key, *new_admin.key);
        admin_info.admin = *new_admin.key;
        admin_info.serialize(&mut &mut admin_info_storage.data.borrow_mut()[..])?;

        msg!("TransferAdmin Done.");

        Ok(())
    }

    pub fn process_verify_sign(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        req_id: MuonRequestId,
        hash: U256Wrap,
        sign: SchnorrSign,
        pub_key: GroupPubKey
    ) -> ProgramResult {

        // Iterating accounts is safer then indexing

        //TODO: should we check account and program_id?

        let ret: bool = schnorr_verify(
            // [U256Wrap] signer x
            pub_key.x.0,
            // [u8] signer y parity
            pub_key.parity,
            // [U256Wrap] signature s
            sign.signature.0,
            // [U256Wrap] msg hash
            hash.0,
            // [U256Wrap] nonce address
            sign.nonce.0
        )?;

        if !ret{
            msg!("TSS Not Verified");
            return Err(MuonError::NotVerified.into());
        }

        msg!("req_id: [{:x}]", req_id);

        //TODO: emit an event.
        // maybe using https://docs.rs/anchor-lang/latest/anchor_lang/macro.emit.html

        Ok(())
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
