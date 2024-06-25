use solana_program::pubkey::Pubkey;


///
/// The mint address of the Token Program this staking program needs.
/// 
pub fn get_natix_token_mint(
) -> Pubkey {    
    Pubkey::try_from("7uyUK8FbXLcZWXPUXUmPSy5Q4C19U6q4kWDUFcV9TtDc").unwrap()
}
