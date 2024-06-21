use std::collections::{binary_heap::Iter, HashSet};

use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::sysvar::Sysvar;
use spl_token::instruction::transfer;
use spl_token::state::Account as TokenAccount;
use crate::staking_info::{StakeInfo, StakePool};


///
/// Returns all the staking transactions.
/// 
pub fn give_stakes(staking_pool: &mut StakePool, address: [u8; 32]) -> Vec<&mut StakeInfo>  {
    
    staking_pool.stakers.iter_mut().filter(|staker| 
        staker.address == address && staker.unstake_time == 0 && staker.stake_time > 0).collect()
    
}

///
/// Returns the count of distinct stakers.
/// 
pub fn count_stakers(staking_pool: &StakePool) -> usize  {
    
    let mut set = HashSet::new();

    for staker in staking_pool.stakers.iter().filter(|staker| 
        staker.unstake_time == 0 && staker.stake_time > 0) {
            set.insert(staker.address);
        }

    set.len()
    
    
}