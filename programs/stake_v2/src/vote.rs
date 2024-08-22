use std::ops::{Div, Mul};

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
use crate::control_flow::{self, control_flow};
use crate::control_max_stakers::control_max_stakers;
use crate::control_provisioned::{self, control_provisioned};
use crate::control_user_solana_address::control_user_solana_address;
use crate::control_voting_origin::control_voting_origin;
use crate::get_reward::{clamped, get_reward};
use crate::give_stakes::give_stakes;
use crate::serialize::serialize;
use crate::staking_info::{Config, Rate, StakeInfo, StakePool};
use crate::transfer_token::transfer_token;
use crate::control_user_account::control_user_account;
use crate::control_stake_amount::control_stake_amount;

///
/// Vote function to vote.
/// 
pub fn vote<'a>(mut staking_pool: StakePool, user_account: &'a AccountInfo<'a>, user_solana_account: &'a AccountInfo<'a>, program_account: &'a AccountInfo<'a>, pda_account: &'a AccountInfo<'a>) -> ProgramResult {            
    msg!("Voting from user account {:?} ", user_account.key);
    control_provisioned(&staking_pool)?;
    control_flow(&staking_pool)?;
    control_user_account(user_account)?;
    control_user_solana_address(user_account, user_solana_account)?;
    control_voting_origin(pda_account)?;

    let config = &staking_pool.config.clone();
    let interests = &staking_pool.interests.clone();
    let stakes = give_stakes(&mut staking_pool, user_account.key.to_bytes());    
    
    let clock = Clock::get()?;

    set_stake_vote(stakes, clock.unix_timestamp, config, interests, true);

    serialize(staking_pool, program_account)
    
}


pub fn set_stake_vote<'a>(mut stakes: Vec<&'a mut StakeInfo>, vote_time: i64, config: &Config, interests: &Vec<Rate>, set_vote: bool) {
    for staker in stakes {

        let mut from = staker.potential_reward_time.min(vote_time);

        let vote_period = config.vote_period_based_on_compound as i64 * config.compound_period as i64 * 3600 * 24;

        if vote_time > staker.vote_time + vote_period {            
            staker.potential_reward += get_reward(interests, config, from, staker.potential_max_reward_time.min(vote_time), staker.amount);
            from = vote_time;            
        }        

        

        staker.potential_reward += get_reward(interests, config, from, vote_time, staker.amount);       

        if set_vote {
            staker.vote_time = vote_time;
            staker.potential_max_reward_time = vote_time + vote_period;
        }
        staker.potential_reward_time = vote_time; 
        
    }    
}