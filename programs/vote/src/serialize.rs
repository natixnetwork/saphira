use borsh::BorshSerialize;
use solana_program::{account_info::AccountInfo, msg};
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::sysvar::Sysvar;
use spl_token::instruction::transfer;
use spl_token::state::Account as TokenAccount;

use crate::data::Data;


///
/// Storing StakePool into program_account.
/// 
pub fn serialize(mut data: Data, program_account: &AccountInfo) -> ProgramResult {
    
    let mut serialized_data = Vec::new();
    data.serialize(&mut serialized_data).unwrap();
    let mut program_data = program_account.try_borrow_mut_data()?;
    let data_length = serialized_data.len();
    if data_length > program_data.len() {
        return Err(ProgramError::AccountDataTooSmall);
    }
    program_data[..data_length].copy_from_slice(&serialized_data);    
    for byte in &mut program_data[data_length..] {
        *byte = 0;
    }
    msg!("Serialized {:?} questions", data.questions.len());
    Ok(())

}