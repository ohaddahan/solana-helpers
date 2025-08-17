use crate::errors::Errors;
use solana_program::account_info::AccountInfo;

/// Provides utilities for transferring SOL and tokens.
pub struct Transfers;

impl Transfers {
    /// Transfers SOL from a PDA account by directly manipulating lamport balances.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Arithmetic overflow occurs during balance calculation
    /// - Unable to borrow lamports for modification
    pub fn transfer_sol_from_pda(
        from: &mut AccountInfo,
        to: &mut AccountInfo,
        amount: u64,
    ) -> Result<(), Errors> {
        let post_from = from
            .lamports()
            .checked_sub(amount)
            .ok_or(Errors::NumericalOverflow)?;
        let post_to = to
            .lamports()
            .checked_add(amount)
            .ok_or(Errors::NumericalOverflow)?;
        **from.try_borrow_mut_lamports()? = post_from;
        **to.try_borrow_mut_lamports()? = post_to;
        Ok(())
    }

    /// Transfers SOL using the system program transfer instruction.
    ///
    /// # Errors
    ///
    /// Returns an error if the program invocation fails
    pub fn transfer_sol<'a>(
        from: AccountInfo<'a>,
        to: AccountInfo<'a>,
        system_program: AccountInfo<'a>,
        amount: u64,
    ) -> Result<(), Errors> {
        solana_program::program::invoke(
            &solana_program::system_instruction::transfer(from.key, to.key, amount),
            &[from, to, system_program],
        )?;
        Ok(())
    }

    /// Transfers SPL tokens between token accounts.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Token instruction creation fails
    /// - Program invocation fails
    pub fn transfer_token<'a>(
        from: AccountInfo<'a>,
        to: AccountInfo<'a>,
        token_program: AccountInfo<'a>,
        owner: AccountInfo<'a>,
        amount: u64,
    ) -> Result<(), Errors> {
        solana_program::program::invoke(
            &spl_token::instruction::transfer(
                token_program.key,
                from.key,
                to.key,
                owner.key,
                &[],
                amount,
            )?,
            &[from, to, token_program, owner],
        )?;
        Ok(())
    }

    /// Transfers SPL tokens from a PDA-owned token account.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Token instruction creation fails
    /// - Program invocation with seeds fails
    pub fn transfer_token_from_pda<'a>(
        from: AccountInfo<'a>,
        to: AccountInfo<'a>,
        token_program: AccountInfo<'a>,
        owner: AccountInfo<'a>,
        amount: u64,
        seeds: &[&[&[u8]]],
    ) -> Result<(), Errors> {
        solana_program::program::invoke_signed(
            &spl_token::instruction::transfer(
                token_program.key,
                from.key,
                to.key,
                owner.key,
                &[],
                amount,
            )?,
            &[from, to, token_program, owner],
            seeds,
        )?;
        Ok(())
    }
}
