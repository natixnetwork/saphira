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
use crate::give_stakes::give_stakes;
use crate::serialize::serialize;
use crate::staking_info::{StakeInfo, StakePool};


///
/// Unstaking all the staked amount of one address. 
/// 
pub fn unstake(mut staking_pool: StakePool, user_account: &AccountInfo, authority_account: &AccountInfo, program_account: &AccountInfo, stake_account: &AccountInfo, token_program_account: &AccountInfo, instant: bool) -> ProgramResult {
    let user_token_account = TokenAccount::unpack(&user_account.data.borrow())?;

    msg!("Unstaking user {:?}, instant: {:?}", user_account.key, instant);
    control_provisioned(&staking_pool)?;
    control_flow(&staking_pool)?;
    control_user_account(user_account)?;
    control_user_solana_address(user_account, authority_account)?;


    let stakes = give_stakes(&mut staking_pool, user_account.key.to_bytes());    

    let clock = Clock::get()?;

    for staker in stakes {
        staker.unstake_time = clock.unix_timestamp;
        staker.instant = instant;
    }

    serialize(staking_pool, program_account)
}