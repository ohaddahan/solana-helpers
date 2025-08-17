use crate::errors::Errors;
use sha2::{Digest, Sha256};
use solana_program::account_info::AccountInfo;
use solana_program::log::sol_log;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

/// Provides generic utility functions for Solana programs.
pub struct Generic;

impl Generic {
    /// Checks if a token mint is the native SOL mint.
    ///
    /// # Arguments
    ///
    /// * `token_mint` - The token mint account to check
    ///
    /// # Returns
    ///
    /// `true` if the mint is the native SOL mint, `false` otherwise
    #[must_use]
    pub fn is_native(token_mint: &AccountInfo) -> bool {
        *token_mint.key == spl_token::native_mint::id()
    }

    /// Burns tokens from a token account using program-derived address authority.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Token instruction creation fails
    /// - Program invocation with seeds fails
    pub fn burn_tokens<'a>(
        token_program: AccountInfo<'a>,
        account: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        authority: AccountInfo<'a>,
        amount: u64,
        seeds: &[&[&[u8]]],
    ) -> Result<(), Errors> {
        let ix = spl_token::instruction::burn(
            token_program.key,
            account.key,
            mint.key,
            authority.key,
            &[authority.key],
            amount,
        )?;
        solana_program::program::invoke_signed(
            &ix,
            &[account, mint, authority, token_program],
            seeds,
        )?;
        Ok(())
    }

    /// Prints a SOL value in human-readable format to the program log.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/label for the value
    /// * `value` - The value in lamports to convert and display
    pub fn print_sol(name: &str, value: u64) {
        #[allow(clippy::cast_precision_loss)]
        sol_log(&format!(
            "{} = {}",
            name,
            value as f64 / LAMPORTS_PER_SOL as f64
        ));
    }

    /// Unsafely clones an `AccountInfo` by transmuting its lifetime to 'static.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it extends the lifetime of the `AccountInfo`
    /// to `'static`, which may lead to use-after-free if the original data is dropped.
    ///
    /// # Arguments
    ///
    /// * `input` - The `AccountInfo` to clone
    ///
    /// # Returns
    ///
    /// A cloned `AccountInfo` with `'static` lifetime
    #[must_use]
    pub fn unsafe_clone_account_info(input: &AccountInfo<'_>) -> AccountInfo<'static> {
        unsafe { std::mem::transmute::<AccountInfo, AccountInfo>(input.clone()) }
    }

    /// Derives an 8-byte discriminator from a string using SHA256.
    ///
    /// # Arguments
    ///
    /// * `name` - The string to derive the discriminator from
    ///
    /// # Returns
    ///
    /// An 8-byte discriminator array
    #[must_use]
    pub fn derive_discriminator(name: &str) -> [u8; 8] {
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        let result = hasher.finalize();
        let mut discriminator = [0u8; 8];
        discriminator.copy_from_slice(&result[..8]);
        discriminator
    }

    /// Creates a program-derived address (PDA) account.
    ///
    /// Validates that the target account matches the expected PDA derived from the seeds,
    /// then creates the account with the specified space and assigns it to the program.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The target account key doesn't match the expected PDA
    /// - Rent calculation fails
    /// - Account creation fails
    pub fn create_pda_account<'a, 'info>(
        target_account: &'a AccountInfo<'info>,
        system_program: &'a AccountInfo<'info>,
        payer: &'a AccountInfo<'info>,
        space: usize,
        program_id: &Pubkey,
        seeds: &[&[u8]],
    ) -> Result<(), Errors> {
        let (expected_pda, bump) = Pubkey::find_program_address(seeds, program_id);
        if *target_account.key != expected_pda {
            return Err(Errors::WrongPdaAddress);
        }
        let bump: &[u8] = &[bump];
        let mut combined_seeds = Vec::with_capacity(seeds.len() + 1);
        combined_seeds.extend_from_slice(seeds);
        combined_seeds.push(bump);
        let seeds = combined_seeds.as_slice();

        let lamports = Rent::get()?.minimum_balance(space);
        let ix = solana_program::system_instruction::create_account(
            payer.key,
            target_account.key,
            lamports,
            space as u64,
            program_id,
        );

        solana_program::program::invoke_signed(
            &ix,
            &[
                payer.clone(),
                target_account.clone(),
                system_program.clone(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}
