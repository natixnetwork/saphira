use std::borrow::BorrowMut;
use std::f64::consts::E;
use std::ops::{Div, Mul};

use crate::control_flow::control_flow;
use crate::control_provisioned::control_provisioned;
use crate::control_user_account::control_user_account;
use crate::control_user_solana_address::control_user_solana_address;
use crate::get_interest_rate::get_interest_rate;
use crate::get_reward::get_reward;
use crate::give_unstakes::give_unstakes;
use crate::serialize::serialize;
use crate::staking_info::{StakeInfo, StakePool};
use crate::transfer_token::{transfer_token, transfer_token_pda};
use crate::{control_flow};
use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;
use spl_token::instruction::transfer;
use spl_token::state::Account as TokenAccount;

///
/// Calculating the reward of all the staking and unstaking requests of one address and transfer it to user account.
/// 
/// If the unstake process is instant it reduces a penalty and then transfer the token to that account.
/// If the unstake requests are not instant then it checks the cooldown time.
/// 
pub fn withdraw<'a>(
    mut staking_pool: StakePool,
    user_account: &'a AccountInfo<'a>,
    authority_account: &'a AccountInfo<'a>,
    program_account: &'a AccountInfo<'a>,
    stake_account: &'a AccountInfo<'a>,
    token_program_account: &'a AccountInfo<'a>,
    pda_account: &'a AccountInfo<'a>,
    program_id: &'a Pubkey,
) -> ProgramResult {
    msg!("Withdrawing for {:?}", user_account.key);
    control_provisioned(&staking_pool)?;
    control_flow(&staking_pool)?;
    control_user_account(user_account)?;
    control_user_solana_address(user_account, authority_account)?;

    let mut clone = staking_pool.clone();
    let unstakes = give_unstakes(&mut clone, user_account.key.to_bytes());

    let clock = Clock::get()?;

    let mut amount: u64 = 0;
    let mut sum_reward: u64 = 0;
    let mut origin_amount: u64 = 0;
    let mut not_calculated_amount: u64 = 0;
    let mut penalty_amount: u64 = 0;
    let mut forfeit_amount: u64 = 0;

    let mut valid_unstakers: Vec<&StakeInfo> = Vec::new();

    for unstaker in unstakes {


        let reward = get_reward(&staking_pool, unstaker);
        let diff_days: f64 = ((clock.unix_timestamp - unstaker.unstake_time) as f64 / (60.0 * 60.0 * 24.0)) as f64;
        if unstaker.instant {
            let penalty = staking_pool.config.instant_penalty.mul(unstaker.amount as f64) as u64;
            amount = amount.checked_add((unstaker.amount.checked_add(reward).expect("Overflow during adding reward").checked_sub(penalty)).unwrap_or(0)).expect("Overflow during summation");
            forfeit_amount = forfeit_amount.checked_add((penalty.checked_sub(reward)).unwrap_or(0)).expect("Overflow during forfeit summation");
            valid_unstakers.push(unstaker);
            sum_reward = sum_reward.checked_add(reward).expect("Overflow during reward sumation");
            origin_amount = origin_amount.checked_add(unstaker.amount).expect("Overflow during origin summation");
            penalty_amount = penalty_amount.checked_add(penalty).expect("Overflow during penalty summation");
        } else if diff_days >= staking_pool.config.cooldown {
            amount = amount.checked_add(unstaker.amount.checked_add(reward).expect("Overflow during adding reward")).expect("Overflow during summation");
            valid_unstakers.push(unstaker);
            sum_reward = sum_reward.checked_add(reward).expect("Overflow during reward sumation");
            origin_amount = origin_amount.checked_add(unstaker.amount).expect("Overflow during origin summation");
        } else {
            not_calculated_amount = not_calculated_amount.checked_add(unstaker.amount).expect("Overflow during adding none calculated summation");
        }
    }

    msg!("Withdrawing amount is {:?}, total reward: {:?}, staked amount: {:?}, and needed to be cooled down amount: {:?}, penalty: {:?}, forfeit amount: {:?}", amount, sum_reward, origin_amount, not_calculated_amount, penalty_amount, forfeit_amount);

    match transfer_token_pda(
        stake_account,
        user_account,
        token_program_account,
        amount,
        pda_account,
        &program_id,
    ) {
        Ok(_) => {
            let stakers: &mut Vec<StakeInfo> = staking_pool.stakers.borrow_mut();                        
            stakers.retain(|staker| !valid_unstakers.contains(&staker));
            staking_pool.forfeit_amount += forfeit_amount;
            serialize(staking_pool, program_account)
        }
        Err(error) => Err(error),
    }
}
