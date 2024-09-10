use std::borrow::BorrowMut;
use std::f64::consts::E;
use std::ops::{Div, Mul};

use crate::control_flow;
use crate::control_flow::control_flow;
use crate::control_owner::control_owner;
use crate::control_provisioned::control_provisioned;
use crate::control_user_account::control_user_account;
use crate::control_user_solana_address::control_user_solana_address;
use crate::get_interest_rate::get_interest_rate;
use crate::get_reward::get_reward;
use crate::give_unstakes::give_unstakes;
use crate::serialize::serialize;
use crate::staking_info::{StakeInfo, StakePool};
use crate::transfer_token::{burn_token_pda, transfer_token, transfer_token_pda};
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
pub fn burn_or_transfer<'a>(
    mut pool: StakePool,
    user_account: &'a AccountInfo<'a>,
    mint_account: &'a AccountInfo<'a>,
    authority_account: &'a AccountInfo<'a>,
    program_account: &'a AccountInfo<'a>,
    stake_account: &'a AccountInfo<'a>,
    token_program_account: &'a AccountInfo<'a>,
    pda_account: &'a AccountInfo<'a>,
    program_id: &'a Pubkey,
) -> ProgramResult {
    msg!(
        "Burn or transfer {:?}, allow_transfer={:?}, allow_burn={:?}",
        pool.forfeit_amount,
        pool.config.forfeit_allow_transfer,
        pool.config.forfeit_allow_burn
    );
    control_provisioned(&pool)?;
    control_user_account(user_account)?;
    control_user_solana_address(user_account, authority_account)?;
    control_owner(authority_account)?;

    if pool.config.forfeit_allow_burn {
        burn_token_pda(
            stake_account,
            mint_account,
            &program_account.key,
            token_program_account,
            pool.forfeit_amount,
            pda_account,
            &program_id,
        )?;
        pool.forfeit_amount = 0;
    } else if pool.config.forfeit_allow_transfer {
        transfer_token_pda(
            stake_account,
            user_account,
            &program_account.key,
            token_program_account,
            pool.forfeit_amount,
            pda_account,
            &program_id,
        )?;
        pool.forfeit_amount = 0;
    } else {
        return Err(ProgramError::Custom(502));
    }
    serialize(pool, program_account)
}
