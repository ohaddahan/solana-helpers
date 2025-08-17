use crate::errors::Errors;
use solana_program::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;

/// Provides validation utilities for Solana programs.
pub struct Validators;

impl Validators {
    /// Validates that an associated token account (ATA) address is correct for the given owner and mint.
    ///
    /// # Arguments
    ///
    /// * `ata` - The ATA address to validate
    /// * `owner` - The expected owner of the ATA
    /// * `mint` - The token mint for the ATA
    ///
    /// # Errors
    ///
    /// Returns `Errors::OwnerMismatch` if the provided ATA doesn't match the expected ATA
    /// for the given owner and mint combination.
    pub fn assert_ata(ata: &Pubkey, owner: &Pubkey, mint: &Pubkey) -> Result<(), Errors> {
        let real_ata = get_associated_token_address(owner, mint);
        if *ata != real_ata {
            return Err(Errors::OwnerMismatch);
        }
        Ok(())
    }
}
