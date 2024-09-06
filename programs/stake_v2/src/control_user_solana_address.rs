use solana_program::pubkey::Pubkey;
use solana_program::{account_info::AccountInfo, msg};
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::sysvar::Sysvar;
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::transfer;
use spl_token::state::Account as TokenAccount;
use crate::get_natix_token_mint::get_natix_token_mint;
use crate::{get_natix_token_id::get_natix_token_id, staking_info::{StakeInfo, StakePool}};

///
/// Controls if the address provided for token account is for the same solana address. 
/// user_solana_account should be signer and address associated for the Token this staking program needs should match the user_account public key provided
/// 
pub fn control_user_solana_address(user_account: &AccountInfo, user_solana_account: &AccountInfo) -> ProgramResult {
    
    msg!("Checking user account {:?} with the provided signer {:?}", user_account.key, user_solana_account.key);
    
    let address = get_associated_token_address(user_solana_account.key, &get_natix_token_mint());

    if user_account.key != &address || user_account.owner != user_solana_account.key {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if !user_solana_account.is_signer {
        return Err(ProgramError::IllegalOwner);
    }

    Ok(())
}