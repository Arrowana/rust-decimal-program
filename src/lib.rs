use rust_decimal::{Decimal, MathematicalOps};
use solana_program::entrypoint;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut input_array = [0u8; 8];
    input_array.copy_from_slice(&instruction_data[..8]);

    let number = u64::from_le_bytes(input_array);
    let input_decimal = Decimal::from(number);

    let sqrt = input_decimal.sqrt().unwrap();

    msg!("Here it is {}", sqrt);

    Ok(())
}
