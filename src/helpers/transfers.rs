use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use crate::errors::Errors;

pub fn transfer_sol_from_pda(
    from: &mut AccountInfo,
    to: &mut AccountInfo,
    amount: u64,
) -> Result<(), ProgramError> {
    let post_from = from
        .lamports()
        .checked_sub(amount)
        .ok_or(Errors::NumericalOverflow)?;
    let post_to = to
        .lamports()
        .checked_add(amount)
        .ok_or(Errors::NumericalOverflow)?;

    **from.try_borrow_mut_lamports().unwrap() = post_from;
    **to.try_borrow_mut_lamports().unwrap() = post_to;
    Ok(())
}

pub fn transfer_sol<'a>(
    from: AccountInfo<'a>,
    to: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    amount: u64,
) -> Result<(), ProgramError> {
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(from.key, to.key, amount),
        &[from, to, system_program],
    )?;
    Ok(())
}

pub fn transfer_token<'a>(
    from: AccountInfo<'a>,
    to: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    owner: AccountInfo<'a>,
    amount: u64,
) -> Result<(), ProgramError> {
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

pub fn transfer_token_pda<'a>(
    from: AccountInfo<'a>,
    to: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    owner: AccountInfo<'a>,
    amount: u64,
    seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
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
