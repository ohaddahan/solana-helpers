use crate::errors::Errors;
use crate::helpers::transfers::Transfers;
use solana_program::account_info::AccountInfo;
use solana_program::program_memory::sol_memset;

/// Provides utilities for closing accounts and transferring lamports.
pub struct Closers;

impl Closers {
    /// Closes a PDA account by transferring all lamports to a destination account.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The transfer of lamports fails
    /// - Unable to borrow account data for memory clearing
    pub fn close_pda(from: &mut AccountInfo, to: &mut AccountInfo) -> Result<(), Errors> {
        let amount = from.lamports();
        let size = from.try_data_len()?;
        Transfers::transfer_sol_from_pda(from, to, amount)?;
        sol_memset(&mut from.try_borrow_mut_data()?, 0, size);
        Ok(())
    }

    /// Closes a regular account by clearing its data, transferring lamports, and reassigning ownership.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Unable to borrow account data for memory clearing
    /// - The SOL transfer fails
    pub fn close_account<'a>(
        from: &AccountInfo<'a>,
        to: AccountInfo<'a>,
        system_program: &AccountInfo<'a>,
    ) -> Result<(), Errors> {
        let amount = from.lamports();
        let size = from.try_data_len()?;
        sol_memset(&mut from.try_borrow_mut_data()?, 0, size);
        Transfers::transfer_sol(from.clone(), to, system_program.clone(), amount)?;
        from.assign(system_program.key);
        Ok(())
    }

    /// Closes a token account using the SPL token program.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Token instruction creation fails
    /// - Program invocation with seeds fails
    pub fn close_token_account<'a>(
        account: AccountInfo<'a>,
        destination: AccountInfo<'a>,
        authority: AccountInfo<'a>,
        seeds: &[&[&[u8]]],
    ) -> Result<(), Errors> {
        let ix = spl_token::instruction::close_account(
            &spl_token::ID,
            account.key,
            destination.key,
            authority.key,
            &[],
        )?;
        solana_program::program::invoke_signed(&ix, &[account, destination, authority], seeds)?;
        Ok(())
    }
}
