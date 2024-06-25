use solana_program::{account_info::AccountInfo, msg};
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;

use crate::data::Data;


///
/// Controls if program account is the signer
/// 
pub fn control_provisioned(data: &Data) -> ProgramResult {
    
    msg!("Checking if account is provisioned {:?}", data.provisioned);
    if !data.provisioned {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}