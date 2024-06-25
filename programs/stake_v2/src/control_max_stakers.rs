use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::sysvar::Sysvar;
use spl_token::instruction::transfer;
use spl_token::state::Account as TokenAccount;
use crate::give_stakes::{count_stakers, give_stakes};
use crate::staking_info::{StakeInfo, StakePool};

///
/// Controls if all distinct stakers are not more than a certain config 
/// 
pub fn control_max_stakers(staking_pool: &StakePool) -> ProgramResult {

    msg!("Checking max stakers {:?}",  staking_pool.config.maximum_stakers);
    

    let stakers_length = count_stakers(staking_pool);
        
    if stakers_length > staking_pool.config.maximum_stakers {
        return Err(ProgramError::Custom(102));
    }

    Ok(())
}