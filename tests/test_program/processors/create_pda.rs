use solana_program::account_info::{AccountInfo, next_account_info};
use solana_program::entrypoint::ProgramResult;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar;
use crate::test_program::state::pda::PDA;

pub fn create_pda(
    accounts: &[AccountInfo],
    bump: u8,
    uuid: String,
    program_id: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer = next_account_info(account_info_iter)?;
    let pda = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent_program = next_account_info(account_info_iter)?;

    assert!(signer.is_signer);
    assert!(signer.is_writable);
    assert!(solana_program::system_program::check_id(system_program.key));
    assert!(sysvar::rent::check_id(rent_program.key));

    let (pda_key, bump_seed) = Pubkey::find_program_address(&[uuid.as_bytes()], program_id);
    assert_eq!(*pda.key, pda_key);
    assert_eq!(bump, bump_seed);

    let mut pda_account = PDA::unpack_unchecked(&pda.try_borrow_data()?)?;
    pda_account.bump = bump;
    pda_account.uuid = uuid;

    PDA::pack(pda_account, &mut pda.try_borrow_mut_data()?)?;

    Ok(())
}