use crate::test_program::instructions::Instructions;
use crate::test_program::processors;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = Instructions::unpack(instruction_data)?;

        match instruction {
            Instructions::CreatePDA { bump, uuid } => {
                msg!("Instruction: CreatePDA");
                Self::process_create_pda(accounts, bump, uuid, program_id)
            }
            Instructions::ClosePDA { bump, uuid } => {
                msg!("Instruction: Exchange");
                Self::process_close_pda(accounts, bump, uuid, program_id)
            }
        }
    }

    fn process_create_pda(
        accounts: &[AccountInfo],
        bump: u8,
        uuid: String,
        program_id: &Pubkey,
    ) -> ProgramResult {
        processors::create_pda::create_pda(accounts, bump, uuid, program_id)
    }

    fn process_close_pda(
        accounts: &[AccountInfo],
        bump: u8,
        uuid: String,
        program_id: &Pubkey,
    ) -> ProgramResult {
        processors::close_pda::close_pda(accounts, bump, uuid, program_id)
    }
}
