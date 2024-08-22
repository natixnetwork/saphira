use solana_program::{account_info::AccountInfo, msg};
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::sysvar::Sysvar;
use spl_token::instruction::transfer;
use spl_token::state::Account as TokenAccount;
use crate::get_natix_token_mint::get_natix_token_mint;
use crate::{get_natix_token_id::get_natix_token_id, staking_info::{StakeInfo, StakePool}};

///
/// It controls if the program is not at pause. 
/// 
pub fn control_flow(pool: &StakePool) -> ProgramResult {
    
    if pool.pause {
        return Err(ProgramError::Custom(501));
    }

    Ok(())
}