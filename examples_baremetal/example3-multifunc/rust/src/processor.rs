use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::functions::{function_a, function_b, function_c};
use crate::instruction::Instruction;

pub struct Processor;
impl Processor {
    pub fn process_program_call(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // turns bytecode into instrucion which contains function to invoke
        let instruction = Instruction::unpack(instruction_data)?;

        msg!("[processor] Received: {:?}", instruction);

        match instruction {
            Instruction::FunctionA => function_a(program_id, accounts, instruction_data),
            Instruction::FunctionB => function_b(program_id, accounts, instruction_data),
            Instruction::FunctionC => function_c(),
        }
    }
}
