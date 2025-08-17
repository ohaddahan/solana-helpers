use crate::errors::Errors;
use solana_program::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;

pub struct Validators;

impl Validators {
    pub fn assert_ata(ata: &Pubkey, owner: &Pubkey, mint: &Pubkey) -> Result<(), Errors> {
        let real_ata = get_associated_token_address(owner, mint);
        if *ata != real_ata {
            return Err(Errors::OwnerMismatch);
        }
        Ok(())
    }
}
