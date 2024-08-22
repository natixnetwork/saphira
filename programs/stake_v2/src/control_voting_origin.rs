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
use crate::get_voting_program_id::get_voting_program_id;
use crate::{get_natix_token_id::get_natix_token_id, staking_info::{StakeInfo, StakePool}};

pub fn control_voting_origin(pda_account: &AccountInfo) -> ProgramResult {
        
    let (pda, _) = Pubkey::find_program_address(&[b"NATIX"], &get_voting_program_id());

    msg!("Checking pda account {:?}, {:?}", pda_account.key, pda);

    if pda != *pda_account.key {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if !pda_account.is_signer {
        return Err(ProgramError::IllegalOwner);
    }

    Ok(())
}