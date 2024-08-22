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
use crate::serialize::serialize;
use crate::staking_info::{StakeInfo, StakePool};
use crate::transfer_token::transfer_token;
use crate::control_user_account::control_user_account;
use crate::control_stake_amount::control_stake_amount;

///
/// Stake function to stake token_amount.
/// 
pub fn stake<'a>(mut staking_pool: StakePool, user_account: &'a AccountInfo<'a>, authority_account: &'a AccountInfo<'a>, program_account: &'a AccountInfo<'a>,
    stake_account: &'a AccountInfo<'a>, token_program_account: &'a AccountInfo<'a>,
             token_amount: u64) -> ProgramResult {            
    msg!("Staking {:?} from user account {:?} to {:?} ", token_amount, user_account.key, stake_account.key);
    control_provisioned(&staking_pool)?;
    control_flow(&staking_pool)?;
    control_user_account(user_account)?;
    control_user_solana_address(user_account, authority_account)?;
    control_stake_amount(&mut staking_pool, user_account.key.to_bytes(), Some(token_amount))?;
    control_max_stakers(&staking_pool)?;

    let clock = Clock::get()?;


    staking_pool.stakers.push(StakeInfo {
        address: user_account.key.to_bytes(),
        amount: token_amount,        
        stake_time: clock.unix_timestamp,
        unstake_time: 0,
        instant: false
    });

    
    match transfer_token(user_account, authority_account, stake_account, token_program_account, token_amount) {
        Ok(_) => {
            serialize(staking_pool, program_account)
        },
        Err(error) => {
            Err(error)
        },
    }

    
}