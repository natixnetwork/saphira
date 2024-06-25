use crate::control_owner::control_owner;
use crate::data::{Data, Question};
use crate::serialize::serialize;
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
/// Set config and restrictions for this staking program.
/// 
/// # It can only be called by the owner of the program_account.
/// 
pub fn remove_expired_questions(
    mut data: Data,    
    program_account: &AccountInfo,    
) -> ProgramResult {
    msg!("Removing expired question");
    
    let clock = Clock::get()?;
    
    
    data.questions.retain(|f| f.expired_at > clock.unix_timestamp);
    serialize(data, program_account)
    
}
