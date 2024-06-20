use std::f64::consts::E;
use std::ops::{Div, Mul};

use crate::control_owner::{self, control_owner};
use crate::control_stake_amount::control_stake_amount;
use crate::control_user_account::control_user_account;
use crate::get_interest_rate::get_interest_rate;
use crate::get_pda::get_pda;
use crate::serialize::serialize;
use crate::staking_info::{Rate, StakeInfo, StakePool};
use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;
use spl_token::instruction::{set_authority, transfer, AuthorityType};
use spl_token::state::Account as TokenAccount;

///
/// Returning the reward for each staking transaction depending on its stake_time and unstake_time.
/// 
/// 
pub fn get_reward<'a>(staking_pool: &StakePool, unstaker: &StakeInfo) -> u64 {
    
    let rates = get_interest_rate(&staking_pool, unstaker.stake_time, unstaker.unstake_time);

    let mut t = unstaker.stake_time;
    let mut p = unstaker.amount as f64;
    let mut r: f64 = if rates.first().is_some() { rates.first().unwrap().amount } else { 0.0 };        
    for rate in rates {        
        
        let diff_days: f64 = ((rate.time - t) as f64 / (60.0 * 60.0 * 24.0)) as f64;
        let clamped = (diff_days as u64).div(staking_pool.config.compound_period as u64).mul(staking_pool.config.compound_period as u64);
        if clamped > 0 {            
            p = p.mul(E.powf(r.mul((clamped as f64).div(365.0))));            
            t = rate.time;
        }        
        r = rate.amount;
    }
    
    let diff_days: f64 = ((unstaker.unstake_time - t) as f64 / (60.0 * 60.0 * 24.0)) as f64;    
    let clamped = (diff_days as u64).div(staking_pool.config.compound_period as u64).mul(staking_pool.config.compound_period as u64);
    
    if clamped > 0 {
        p = p.mul(E.powf(r.mul((clamped as f64).div(365.0))));                
    }

    ((p as u64) - unstaker.amount).max(0)
}
