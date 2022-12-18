use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use solana_program::program_memory::sol_memset;
use crate::helpers::transfers::{transfer_sol, transfer_sol_from_pda};

pub fn close_pda(from: &mut AccountInfo, to: &mut AccountInfo) -> Result<(), ProgramError> {
    let amount = from.lamports();
    let size = from.try_data_len()?;
    transfer_sol_from_pda(from, to, amount)?;
    sol_memset(&mut from.try_borrow_mut_data()?, 0, size);
    Ok(())
}

pub fn close_account<'a>(from: AccountInfo<'a>, to: AccountInfo<'a>, system_program: AccountInfo<'a>) -> Result<(), ProgramError> {
    let amount = from.lamports();
    let size = from.try_data_len()?;
    sol_memset(&mut from.try_borrow_mut_data()?, 0, size);
    transfer_sol(from.clone(), to,  system_program.clone(), amount)?;
    from.assign(system_program.key);
    Ok(())
}
