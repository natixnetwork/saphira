use crate::control_owner::control_owner;
use crate::control_stake_amount::control_stake_amount;
use crate::control_user_account::control_user_account;
use crate::get_pda::{get_pda};
use crate::serialize::serialize;
use crate::staking_info::{StakeInfo, StakePool};
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
/// This function pauses the staking program, and it doesn't accept any request after that.
/// 
/// # It can only be called by the owner of program_account.
/// 
pub fn pause(
    mut pool: StakePool,    
    authority_account: &AccountInfo,
    program_account: &AccountInfo
) -> ProgramResult {    
    msg!("Pausing the contract!");

    control_owner(authority_account)?;

    pool.pause = true;
    serialize(pool, program_account)    
}
