use std::ops::Div;

use crate::control_owner::{self, control_owner};
use crate::control_stake_amount::control_stake_amount;
use crate::control_user_account::control_user_account;
use crate::get_natix_token_id::get_natix_token_id;
use crate::get_pda::{get_pda};
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
/// Adds another interest rate.
/// 
/// # Only owner of program account can call this.
/// 
pub fn change_interest_rate<'a>(
    mut pool: StakePool,
    program_account: &'a AccountInfo<'a>,    
    authority_account: &'a AccountInfo<'a>,    
    rate: u64
) -> ProgramResult {
    msg!("Changing interest rate to {:?} for {:?}", rate, program_account.key);

    control_owner(authority_account)?;

    let clock = Clock::get()?;

    let amount = (rate as f64).div(100.0);

    if amount > 1f64 || amount < 0f64 {
        return Err(ProgramError::InvalidArgument);
    }

    pool.interests.push(Rate {
        amount: amount,
        time: clock.unix_timestamp
    });

    serialize(pool, program_account)
}
