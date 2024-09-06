use crate::staking_info::{Config, Rate, StakeInfo, StakePool};
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


///
/// Reads the program_account and returns StakePool struct
/// 
pub fn deserialize(program_account: &AccountInfo) -> StakePool {
    let mut data: &[u8] = &program_account.data.borrow();
    let data_ref: &mut &[u8] = &mut data;

    let pool = StakePool::deserialize(data_ref);
    pool.unwrap_or(StakePool {
        stakers: Vec::new(),
        pause: false,
        interests: vec![
            Rate {
            amount: 0.01,
            time: 0
        }],
        config: Config {
            min_stake_amount: 100000000,
            max_stake_amount: 10000000000,
            cooldown: 90.0,
            instant_penalty: 0.3,
            maximum_stakers: 100,
            compound_period: 7,
            forfeit_allow_burn: false,
            forfeit_allow_transfer: false,
            stake_account: None,
            program_account: None,
        },
        forfeit_amount: 0,
        provisioned: false
    })    
}
