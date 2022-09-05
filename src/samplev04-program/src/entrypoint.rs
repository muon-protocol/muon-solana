use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey
};

use crate::processor::Processor;
use crate::solana_program_runtime::ComputeBudgetInstruction

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
	let instruction = ComputeBudgetInstruction::set_compute_unit_limit(300_000);
    Processor::process(program_id, accounts, instruction_data)
}
