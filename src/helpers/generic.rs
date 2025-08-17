use crate::errors::Errors;
use sha2::{Digest, Sha256};
use solana_program::account_info::AccountInfo;
use solana_program::log::sol_log;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub struct Generic;

impl Generic {
    pub fn is_native(token_mint: &AccountInfo) -> bool {
        *token_mint.key == spl_token::native_mint::id()
    }

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
            &[&authority.key],
            amount,
        )?;
        solana_program::program::invoke_signed(
            &ix,
            &[account, mint, authority, token_program],
            seeds,
        )?;
        Ok(())
    }

    pub fn print_sol(name: &str, value: u64) {
        sol_log(&format!(
            "{} = {}",
            name,
            value as f64 / LAMPORTS_PER_SOL as f64
        ));
    }

    pub fn unsafe_clone_account_info<'a, 'info>(
        input: &'a AccountInfo<'info>,
    ) -> AccountInfo<'static> {
        unsafe { std::mem::transmute::<AccountInfo, AccountInfo>(input.clone()) }
    }

    pub fn derive_discriminator(name: &str) -> [u8; 8] {
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        let result = hasher.finalize();
        let mut discriminator = [0u8; 8];
        discriminator.copy_from_slice(&result[..8]);
        discriminator
    }

    #[inline(always)]
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
