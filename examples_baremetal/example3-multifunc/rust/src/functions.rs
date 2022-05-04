use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    pubkey::Pubkey,
};

pub fn function_a(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("[functions] function_A reporting!");

    let account_info_iter = &mut accounts.iter();
    let helloworld_account = next_account_info(account_info_iter)?;

    // assemble new instructions
    let inst = Instruction::new_with_bincode(*helloworld_account.key, &[0; 0], vec![]);

    msg!("[entrypoint] Calling helloworld");

    // invoke helloworld
    invoke(&inst, &[])?;

    Ok(())
}

pub fn function_b(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("[functions] function_B reporting!");

    let account_info_iter = &mut accounts.iter();
    let counter_program = next_account_info(account_info_iter)?;
    let counter_account = next_account_info(account_info_iter)?;

    let account_metas = vec![AccountMeta::new(*counter_account.key, false)];
    // assemble new instructions
    let inst = Instruction::new_with_bincode(*counter_program.key, &[0; 0], account_metas);

    msg!("[entrypoint] Calling counter");

    // invoke counter
    invoke(&inst, &[counter_account.clone(), counter_account.clone()])?;

    Ok(())
}

pub fn function_c() -> ProgramResult {
    msg!("[functions] function_C reporting!");

    Ok(())
}
