use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;
use spl_token::instruction::transfer;
use spl_token::state::Account as TokenAccount;
use crate::serialize::serialize;
use crate::staking_info::{StakeInfo, StakePool};
use crate::control_user_account::control_user_account;
use crate::control_stake_amount::control_stake_amount;


///
/// Returns the pda address, this address should be the owner of stake_acount.
/// So that it can transfer token from stake_account to users' accounts.
/// 
pub fn get_pda(program_id: &Pubkey, program_account_pub_key: &Pubkey, pda_account: &AccountInfo) -> Result<(Pubkey, u8), ProgramError> {
    
    let seed = program_account_pub_key.to_bytes();

    let (pda, bump_seed) = Pubkey::find_program_address(&[&seed], program_id);

    if pda != *pda_account.key {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok((pda, bump_seed))
}