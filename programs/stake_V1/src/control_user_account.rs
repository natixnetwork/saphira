use solana_program::pubkey::Pubkey;
use solana_program::{account_info::AccountInfo, msg};
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::sysvar::Sysvar;
use spl_token::instruction::transfer;
use spl_token::state::Account as TokenAccount;
use crate::get_natix_token_id::get_natix_token_id;
use crate::get_natix_token_mint::get_natix_token_mint;
use crate::{staking_info::{StakeInfo, StakePool}};

///
/// Controls if the address provided for token account is for the token this staking program needs. 
/// 
pub fn control_user_account(user_account: &AccountInfo) -> ProgramResult {
    let user_token_account = TokenAccount::unpack(&user_account.data.borrow())?;
    
    msg!("Checking user token account mint address {:?}, {:?}", user_token_account.mint, user_token_account.owner);
    
    if user_token_account.mint != get_natix_token_mint() {
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(())
}