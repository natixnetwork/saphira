use borsh::BorshSerialize;
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
use crate::control_flow::control_flow;
use crate::control_provisioned::control_provisioned;
use crate::control_user_account::control_user_account;
use crate::control_user_solana_address::control_user_solana_address;
use crate::get_reward::get_reward;
use crate::give_stakes::give_stakes;
use crate::serialize::serialize;
use crate::staking_info::{Config, Rate, StakeInfo, StakePool};
use crate::vote::set_stake_vote;


///
/// Unstaking all the staked amount of one address. 
/// 
pub fn unstake(mut staking_pool: StakePool, user_account: &AccountInfo, authority_account: &AccountInfo, program_account: &AccountInfo, stake_account: &AccountInfo, token_program_account: &AccountInfo, instant: bool) -> ProgramResult {    

    msg!("Unstaking user {:?}, instant: {:?}", user_account.key, instant);
    control_provisioned(&staking_pool)?;
    control_flow(&staking_pool)?;
    control_user_account(user_account)?;
    control_user_solana_address(user_account, authority_account)?;

    let config = &staking_pool.config.clone();
    let interests = &staking_pool.interests.clone();    

    let clock = Clock::get()?;
    
    let stakes = give_stakes(&mut staking_pool, user_account.key.to_bytes());
    unstake_staker(stakes, clock.unix_timestamp, instant, config, interests);

    serialize(staking_pool, program_account)
}

pub fn unstake_staker(mut stakes: Vec<&mut StakeInfo>, vote_time: i64, instant: bool, config: &Config, interests: &Vec<Rate>) {                
    for staker in stakes {
        staker.unstake_time = vote_time;
        staker.instant = instant;        

        if staker.vote_time > 0 {
            set_stake_vote(vec![staker], vote_time, config, interests, false);
        }

    }
}