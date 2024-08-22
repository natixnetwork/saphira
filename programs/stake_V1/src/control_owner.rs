use solana_program::{account_info::AccountInfo, msg};
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;

use crate::get_owner_id::get_owner_id;


///
/// Controls if program account is the signer
/// 
pub fn control_owner(owner_account: &AccountInfo) -> ProgramResult {
    
    msg!("Checking owner account key {:?}", owner_account.key);
    if !owner_account.is_signer || *owner_account.key != get_owner_id(){
        return Err(ProgramError::IllegalOwner);
    }

    Ok(())
}