use solana_program::account_info::AccountInfo;

pub fn is_native(token_mint: &AccountInfo) -> bool {
    *token_mint.key == spl_token::native_mint::id()
}
