use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    pubkey::Pubkey,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey,     // Public key of the account (unused)
    accounts: &[AccountInfo], // Account/Program ID of the helloworld program
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("[entrypoint] CPI");

    // Get passed program ID of the helloworld
    let account_info_iter = &mut accounts.iter();
    let counter_program = next_account_info(account_info_iter)?;
    let counter_account = next_account_info(account_info_iter)?;

    let account_metas = vec![AccountMeta::new(*counter_account.key, false)];
    // assemble new instructions
    let inst = Instruction::new_with_bincode(*counter_program.key, &[0; 0], account_metas);

    msg!("[entrypoint] Calling counter");

    // invoke helloworld
    invoke(&inst, &[counter_program.clone(), counter_account.clone()])?;

    Ok(())
}
