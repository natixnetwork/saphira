use crate::control_owner::{self, control_owner};
use crate::control_stake_amount::control_stake_amount;
use crate::control_user_account::control_user_account;
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
/// Returns all the rates appropriate for the time between from_time and to_time.
/// 
pub fn get_interest_rate<'a>(
    pool: &StakePool,
    from_time: i64,
    to_time: i64,
) -> Vec<&Rate> {
    let mut before_from_rates: Vec<&Rate> = pool.interests.iter().filter(|rate| rate.time < from_time).collect();
    before_from_rates.sort_by(|a, b| a.time.cmp(&b.time));
    let mut res: Vec<&Rate> = vec![];
    if before_from_rates.last().is_some() {
        res.push(before_from_rates.last().unwrap());
    }    
    let rates: Vec<&Rate> = pool.interests.iter().filter(|rate| from_time <= rate.time && rate.time <= to_time).collect();  
    res.extend(rates);
    res
}
