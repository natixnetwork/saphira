use solana_program::{account_info::AccountInfo, msg};
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;

use crate::staking_info::StakePool;


///
/// Controls if program account is the signer
/// 
pub fn control_provisioned(staking_pool: &StakePool) -> ProgramResult {
    
    msg!("Checking if account is provisioned {:?}", staking_pool.provisioned);
    if !staking_pool.provisioned {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}