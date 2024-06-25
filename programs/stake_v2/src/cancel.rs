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
use crate::control_max_stakers::control_max_stakers;
use crate::control_provisioned::control_provisioned;
use crate::control_stake_amount::control_stake_amount;
use crate::control_user_account::control_user_account;
use crate::control_user_solana_address::control_user_solana_address;
use crate::give_stakes::give_stakes;
use crate::give_unstakes::give_unstakes;
use crate::serialize::serialize;
use crate::staking_info::{StakeInfo, StakePool};


///
/// Cancelling all the unstake for this address
/// 
pub fn cancel(mut staking_pool: StakePool, user_account: &AccountInfo, program_account: &AccountInfo, user_solana_address: &AccountInfo) -> ProgramResult {
    
    msg!("Cancelling all requests for user {:?}", user_account.key);
    control_provisioned(&staking_pool)?;
    control_flow(&staking_pool)?;
    control_user_account(user_account)?;
    control_user_solana_address(user_account, user_solana_address)?;


    let stakes = give_unstakes(&mut staking_pool, user_account.key.to_bytes());    
    
    for staker in stakes {
        staker.unstake_time = 0;
        staker.instant = false;
    }
    
    control_stake_amount(&mut staking_pool, user_account.key.to_bytes(), None)?;

    serialize(staking_pool, program_account)
}