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
use crate::give_stakes::give_stakes;
use crate::staking_info::{StakeInfo, StakePool};

///
/// Controls total amount staked by a certain address is between min_stake_amount and max_stake_amount
/// 
pub fn control_stake_amount(staking_pool: &mut StakePool, address: [u8; 32], amount: Option<f64>) -> Result<u64, ProgramError> {

    msg!("Checking amount {:?}, min: {:?}, max: {:?}", amount, staking_pool.config.min_stake_amount, staking_pool.config.max_stake_amount);

    if amount.is_some() {
        if amount.unwrap() < staking_pool.config.min_stake_amount as f64 || amount.unwrap() > staking_pool.config.max_stake_amount as f64 {
            return Err(ProgramError::Custom(101));
        }
    }

    let token_amount = amount.unwrap_or(0.0) as u64;

    let sum_stakes = token_amount.checked_add(
        give_stakes(staking_pool, address)
            .iter()
            .fold(Some(0u64), |acc, staker| {
                acc.and_then(|sum| sum.checked_add(staker.amount))
            })
            .expect("Overflow during stake summation")
    ).expect("Overflow during final addition");
    
    msg!("Checking sum amount for this address, sum: {:?}", sum_stakes);
    if sum_stakes > staking_pool.config.max_stake_amount {
        return Err(ProgramError::Custom(102));
    }

    Ok(token_amount)
}