use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
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
/// Reads the program_account and returns StakePool struct
/// 
pub fn deserialize(program_account: &AccountInfo) -> Data {
    let mut data: &[u8] = &program_account.data.borrow();
    let data_ref: &mut &[u8] = &mut data;

    let pool = Data::deserialize(data_ref);
    match pool {
        Ok(p) => p,
        Err(e) => Data {
            questions: vec![],
            provisioned: false,
        },
    }
}
