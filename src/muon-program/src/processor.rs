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
    types::{U256Wrap, SchnorrSign, MuonRequestId},
    instructions::MuonInstruction,
    state::{GroupInfo, AdminInfo},
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
        let instruction = MuonInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            MuonInstruction::InitializeAdmin => {
                Self::process_initialize_admin(program_id, accounts)
            }
            MuonInstruction::TransferAdmin => {
                Self::process_transfer_admin(program_id, accounts)
            }
            MuonInstruction::AddGroup { eth_address, pubkey_x, pubkey_y_parity } => {
                Self::process_add_group(program_id, accounts, eth_address, pubkey_x, pubkey_y_parity)
            }
            MuonInstruction::VerifySignature { req_id, hash, sign} => {
                Self::process_verify_sign(program_id, accounts, req_id, hash, sign)
            }
        }
    }

    pub fn process_initialize_admin(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        msg!("InitializeAdmin start");

        // TODO: is need to program KeyPair sign this transaction?

        // Iterating accounts is safer then indexing
        let accounts_iter = &mut accounts.iter();

        // Get the account to store admin info
        let admin_info_storage = next_account_info(accounts_iter)?;

        Self::validate_admin_storage(program_id, admin_info_storage)?;

        // Increment and store the number of times the account has been greeted
        let mut admin_info = AdminInfo::try_from_slice(&admin_info_storage.data.borrow())?;
        //    msg!("TransferAdmin from {} to {}...", admin_info.admin, admin);

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
        msg!("TransferAdmin start");

        // Iterating accounts is safer then indexing
        let accounts_iter = &mut accounts.iter();

        // Get the account to store admin info
        let admin_info_storage = next_account_info(accounts_iter)?;

        Self::validate_admin_storage(program_id, admin_info_storage)?;

        // Increment and store the number of times the account has been greeted
        let mut admin_info = AdminInfo::try_from_slice(&admin_info_storage.data.borrow())?;
        //    msg!("TransferAdmin from {} to {}...", admin_info.admin, admin);

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

    pub fn process_add_group(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        eth_address: U256Wrap,
        pubkey_x: U256Wrap,
        pubkey_y_parity: u8,
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
            msg!("admin is not signer.");
            return Err(MuonError::MissingAdminSignature.into());
        }

        let mut group_info = GroupInfo::try_from_slice(&group_info_storage.data.borrow())?;

        group_info.is_valid = true;
        group_info.eth_address = eth_address;
        group_info.pubkey_x = pubkey_x;
        group_info.pubkey_y_parity = pubkey_y_parity;
        group_info.serialize(&mut &mut group_info_storage.data.borrow_mut()[..])?;

        msg!("AddGroup Done.");

        Ok(())
    }

    pub fn process_verify_sign(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        req_id: MuonRequestId,
        hash: U256Wrap,
        sign: SchnorrSign
    ) -> ProgramResult {

        // Iterating accounts is safer then indexing
        let accounts_iter = &mut accounts.iter();

        // Get the account to store admin info
        let group_info_storage = next_account_info(accounts_iter)?;

        let group_info = GroupInfo::try_from_slice(&group_info_storage.data.borrow())?;

        // The account must be owned by the program in order to modify its data
        if group_info_storage.owner != program_id || !group_info.is_initialized() {
            msg!("GroupInfo account does not have the correct program id");
            return Err(MuonError::InvalidGroupAccountOwner.into());
        }

        if !group_info.is_valid {
            msg!("group_info is not valid");
            return Err(MuonError::NotVerified.into());
        }

        if group_info.eth_address != sign.address {
            msg!("sign.address not matched with group_info.eth_address");
            return Err(MuonError::NotVerified.into());
        }

        schnorr_verify(
            // [U256Wrap] signer x
            group_info.pubkey_x.0,
            // [u8] signer y parity
            group_info.pubkey_y_parity,
            // [U256Wrap] signature s
            sign.signature.0,
            // [U256Wrap] msg hash
            hash.0,
            // [U256Wrap] nonce address
            sign.nonce.0
        )?;

        msg!("req_id: [{:x}]", req_id);

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
