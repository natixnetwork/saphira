pub mod cancel;
pub mod change_interest_rate;
pub mod control_flow;
pub mod control_max_stakers;
pub mod control_owner;
pub mod control_provisioned;
pub mod control_stake_amount;
pub mod control_user_account;
pub mod control_user_solana_address;
pub mod data;
pub mod deserialize;
pub mod get_interest_rate;
pub mod get_natix_token_id;
pub mod get_natix_token_mint;
pub mod get_owner_id;
pub mod get_pda;
pub mod get_reward;
pub mod give_stakes;
pub mod give_unstakes;
pub mod init;
pub mod instruction;
pub mod pause;
pub mod resume;
pub mod serialize;
pub mod set_config;
pub mod stake;
pub mod staking_info;
pub mod transfer_token;
pub mod unstake;
pub mod withdraw;
pub mod burn_or_transfer;

use crate::cancel::cancel;
use crate::change_interest_rate::change_interest_rate;
use crate::data::Data;
use crate::deserialize::deserialize;
use crate::init::init;
use crate::instruction::ProgramInstruction;
use crate::serialize::serialize;
use crate::set_config::set_config;
use crate::stake::stake;
use crate::staking_info::{StakeInfo, StakePool};
use crate::unstake::unstake;
use crate::withdraw::withdraw;
use crate::ProgramInstruction::{Stake, Withdraw};
use borsh::{BorshDeserialize, BorshSerialize};
use burn_or_transfer::burn_or_transfer;
use get_natix_token_id::get_natix_token_id;
use get_natix_token_mint::get_natix_token_mint;
use pause::pause;
use resume::resume;
use solana_program::account_info::next_account_info;
use solana_program::clock::Clock;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::sysvar::Sysvar;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};
use spl_token::instruction::transfer;
use spl_token::state::Account as TokenAccount;
use std::borrow::Borrow;
use std::str::FromStr;

entrypoint!(process_instruction);

// also please ensure the correct get_program_account_id, get_stake_account_id, get_natix_token_mint

pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &'a [u8],
) -> ProgramResult {
    msg!("NATIX Staking Program!");
    let account_info_iter = &mut accounts.iter();

    let program_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
    let stake_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
    let token_program_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;

    msg!("Checking program id {:?}", program_account.owner);
    if program_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    if *token_program_account.key != get_natix_token_id() {
        return Err(ProgramError::InvalidArgument);
    }

    msg!("Checking stake token account owner {:?}", stake_account.owner);

    if *stake_account.owner != get_natix_token_id() {
        return Err(ProgramError::InvalidArgument);
    }
    let stake_token_account = TokenAccount::unpack(&stake_account.data.borrow_mut())?;
    let seed = program_account.key.to_bytes();
    let (pda, _bump_seed) = Pubkey::find_program_address(&[&seed], program_id);
    msg!(
        "Checking stake account owner {:?}, {:?}",
        stake_token_account.owner,
        pda
    );
    if stake_token_account.owner != pda {
        return Err(ProgramError::InvalidArgument);
    }

    msg!(
        "Checking stake account mint {:?}, {:?}",
        stake_token_account.mint,
        get_natix_token_mint()
    );
    if stake_token_account.mint != get_natix_token_mint() {
        return Err(ProgramError::InvalidArgument);
    }    

    let pool = deserialize(program_account);

    let stake_defined = &pool.config.stake_account;
    let program_defined = &pool.config.program_account;


    msg!(
        "Checking stake account {:?}, {:?}",
        stake_account.key,
        stake_defined
    );

    if stake_defined.is_some() {
        let account = Pubkey::from_str(stake_defined.clone().unwrap().as_str()).map_err(|_| ProgramError::InvalidArgument)?;       
        if *stake_account.key != account {
            return Err(ProgramError::InvalidArgument);
        }
    }

    msg!(
        "Checking program account {:?}, {:?}",
        program_account.key,
        program_defined
    );

    if program_defined.is_some() {
        let account = Pubkey::from_str(program_defined.clone().unwrap().as_str()).map_err(|_| ProgramError::InvalidArgument)?;        
        if *program_account.key != account {
            return Err(ProgramError::InvalidArgument);
        }
    }

    let data = Data::try_from_slice(instruction_data).map_err(|_| ProgramError::InvalidArgument)?;

    let instruction = data.instruction;
    match instruction {
        ProgramInstruction::Init => {
            let authority_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            return init(pool, authority_account, program_account);
        }
        ProgramInstruction::ChangeInterest => {
            let authority_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            return change_interest_rate(pool, program_account, authority_account, data.amount);
        }
        ProgramInstruction::Config => {
            let authority_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            if data.config.is_some() {
                let mut config = data.config.unwrap();
                config.stake_account = Some(stake_account.key.to_string());
                config.program_account = Some(program_account.key.to_string());
                return set_config(pool, authority_account, program_account, Some(config));
            }
            return set_config(pool, authority_account, program_account, data.config);
        }
        ProgramInstruction::Stake => {
            let user_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            let authority_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            return stake(
                pool,
                user_account,
                authority_account,
                program_account,
                stake_account,
                token_program_account,
                data.amount,
            );
        }
        ProgramInstruction::UnStake => {
            let user_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            let authority_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            return unstake(
                pool,
                user_account,
                authority_account,
                program_account,
                stake_account,
                token_program_account,
                false,
            );
        }
        ProgramInstruction::UnStakeInstant => {
            let user_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            let authority_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            return unstake(
                pool,
                user_account,
                authority_account,
                program_account,
                stake_account,
                token_program_account,
                true,
            );
        }
        ProgramInstruction::Cancel => {
            let user_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            let user_solana_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            return cancel(pool, user_account, program_account, user_solana_account);
        }
        ProgramInstruction::Withdraw => {
            let user_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            let pda_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            let authority_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            return withdraw(
                pool,
                user_account,
                authority_account,
                program_account,
                stake_account,
                token_program_account,
                pda_account,
                program_id,
            );
        }
        ProgramInstruction::Pause => {
            let authority_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            return pause(pool, authority_account, program_account);
        }
        ProgramInstruction::Resume => {
            let authority_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            return resume(pool, authority_account, program_account);
        }
        ProgramInstruction::BurnOrTransferForfeit => {
            let user_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            let mint_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            let pda_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            let authority_account: &'a AccountInfo<'a> = next_account_info(account_info_iter)?;
            return burn_or_transfer(pool,  
                user_account,    
                mint_account,          
                authority_account,
                program_account,
                stake_account,
                token_program_account,
                pda_account,
                program_id,);
        }        
    }
}
