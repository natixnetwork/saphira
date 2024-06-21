use crate::control_owner::control_owner;
use crate::serialize::serialize;
use crate::staking_info::{StakeInfo, StakePool};
use borsh::BorshSerialize;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::sysvar::Sysvar;
use solana_program::{account_info::AccountInfo, msg};
use spl_token::instruction::transfer;
use spl_token::state::Account as TokenAccount;

///
/// Store the defualt StakePool struct into the program_account
/// 
/// # This function can only be called by the owner of program_account.
/// 
pub fn init(mut staking_pool: StakePool, authority_account: &AccountInfo, program_account: &AccountInfo) -> ProgramResult {
    msg!("Initializing contract!");
    control_owner(authority_account)?;
    serialize(staking_pool, program_account)
}
