use std::str::FromStr;

use crate::control_owner::control_owner;
use crate::get_pda::get_pda;
use crate::serialize::serialize;
use crate::staking_info::{Config, StakeInfo, StakePool};
use borsh::BorshSerialize;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;
use solana_program::{account_info::AccountInfo, msg};
use spl_token::instruction::transfer;
use spl_token::state::Account as TokenAccount;


///
/// Set config and restrictions for this staking program.
/// 
/// # It can only be called by the owner of the program_account.
/// 
pub fn set_config(
    mut staking_pool: StakePool,
    authority_account: &AccountInfo,
    program_account: &AccountInfo,
    config: Option<Config>,    
) -> ProgramResult {
    msg!("Setting config contract {:?}", config);
    control_owner(authority_account)?;

    match config {
        Some(c) => {
            if c.instant_penalty > 1f64 || c.instant_penalty < 0f64 {
                return Err(ProgramError::InvalidArgument);
            }            
            if c.stake_account.is_some() ^ c.program_account.is_some()  {
                return Err(ProgramError::InvalidArgument);
            }
            staking_pool.config = c;
            staking_pool.provisioned = true;
            serialize(staking_pool, program_account)
        }
        None => Err(ProgramError::InvalidArgument),
    }
}
