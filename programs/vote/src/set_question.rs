use crate::control_owner::control_owner;
use crate::data::{Data, Question};
use crate::remove_expired_questions::remove_expired_questions;
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
pub fn set_questions(
    mut data: Data,
    authority_account: &AccountInfo,
    program_account: &AccountInfo,
    question: Option<Question>,
) -> ProgramResult {
    msg!("Setting question {:?}", question);
    control_owner(authority_account)?;
    let clock = Clock::get()?;
    
    match question {
        Some(c) => {
            data.questions.push(c);
            data.provisioned = true;
            remove_expired_questions(data, program_account)
        }
        None => Err(ProgramError::InvalidArgument),
    }
}
