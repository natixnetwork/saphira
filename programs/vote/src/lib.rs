pub mod instruction;
pub mod data;
pub mod serialize;
pub mod deserialize;
pub mod control_owner;
pub mod init;
pub mod set_question;
pub mod get_owner_id;
pub mod control_provisioned;
pub mod vote;
pub mod remove_expired_questions;
pub mod control_user_solana_address;
pub mod get_natix_token_mint;

use std::borrow::Borrow;
use std::str::FromStr;
use borsh::{BorshDeserialize, BorshSerialize};
use data::Input;
use set_question::set_questions;
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};
use solana_program::account_info::next_account_info;
use solana_program::clock::Clock;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::sysvar::Sysvar;
use spl_token::instruction::transfer;
use crate::data::Data;
use crate::init::init;
use crate::instruction::ProgramInstruction;
use crate::serialize::serialize;
use crate::vote::vote;
use spl_token::state::Account as TokenAccount;
use crate::deserialize::deserialize;


entrypoint!(process_instruction);




// also please ensure the correct get_program_account_id, get_stake_account_id, get_natix_token_id, get_natix_token_mint

pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &'a [u8],
) -> ProgramResult {
    
    msg!("NATIX Staking Program!");
    let account_info_iter = &mut accounts.iter();    

    let program_account: &'a AccountInfo<'a > = next_account_info(account_info_iter)?;        
    
    msg!("Checking program id {:?}", program_account.owner);
    if program_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }
    
    
    let data = deserialize(program_account);

    
    match Input::try_from_slice(instruction_data) {
        Ok(input) => {
            let instruction = input.instruction;
            match instruction {
                ProgramInstruction::Init => { 
                    let authority_account: &'a AccountInfo<'a > = next_account_info(account_info_iter)?;
                    return init(data, authority_account, program_account);
                }
                ProgramInstruction::Set => {                       
                    let authority_account: &'a AccountInfo<'a > = next_account_info(account_info_iter)?;
                    return set_questions(data, authority_account, program_account, input.question);
                }            
                ProgramInstruction::Vote => {   
                    let user_account: &'a AccountInfo<'a > = next_account_info(account_info_iter)?;                    
                    let authority_account: &'a AccountInfo<'a > = next_account_info(account_info_iter)?;
                    let stake_program_id: &'a AccountInfo<'a > = next_account_info(account_info_iter)?;
                    let token_program_account: &'a AccountInfo<'a > = next_account_info(account_info_iter)?;
                    let stake_program_account: &'a AccountInfo<'a > = next_account_info(account_info_iter)?;
                    let stake_account: &'a AccountInfo<'a > = next_account_info(account_info_iter)?;
                    return vote(data, 
                        user_account, 
                        authority_account, 
                        program_account, 
                        stake_program_id, 
                        token_program_account, 
                        stake_program_account, 
                        stake_account, 
                        input.questionId.unwrap(), 
                        input.answerId.unwrap());
                }
                _ => {
                    return Err(ProgramError::InvalidArgument);
                }
            }
        }
        _ => { 
            return Err(ProgramError::InvalidArgument);
        }
    }
}
