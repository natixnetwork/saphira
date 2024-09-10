use crate::control_stake_amount::control_stake_amount;
use crate::control_user_account::control_user_account;
use crate::get_pda::{get_pda};
use crate::serialize::serialize;
use crate::staking_info::{StakeInfo, StakePool};
use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;
use spl_token::instruction::{set_authority, transfer, AuthorityType};
use spl_token::state::Account as TokenAccount;

///
/// The mint address of the Token Program this staking program needs.
/// 
pub fn get_natix_token_mint(
) -> Pubkey {    
    Pubkey::try_from("7uyUK8FbXLcZWXPUXUmPSy5Q4C19U6q4kWDUFcV9TtDc").unwrap()
    // Pubkey::try_from("FRySi8LPkuByB7VPSCCggxpewFUeeJiwEGRKKuhwpKcX").unwrap()
}
