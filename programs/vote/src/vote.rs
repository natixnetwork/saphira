use borsh::to_vec;
use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::msg;
use solana_program::program::invoke;
use solana_program::sysvar::Sysvar;
use crate::control_provisioned::{self, control_provisioned};
use crate::control_user_solana_address::control_user_solana_address;
use crate::data::{Data, Question, StakeInstruction};
use crate::remove_expired_questions::remove_expired_questions;

///
/// Vote function to vote.
/// 
pub fn vote<'a>(mut data: Data, user_account: &'a AccountInfo<'a>, 
user_solana_account: &'a AccountInfo<'a>, 
program_account: &'a AccountInfo<'a>,
stake_program_id: &'a AccountInfo<'a>,
token_program_account: &'a AccountInfo<'a>,
stake_program_account: &'a AccountInfo<'a>,
stake_account: &'a AccountInfo<'a>,
questionId: String, 
answerId: String) -> ProgramResult {            
    msg!("Voting from user account {:?} ", user_account.key);
    control_provisioned(&data)?;        
    control_user_solana_address(user_account, user_solana_account)?;    

    let clock = Clock::get()?;

    let questions: Vec<&mut Question> = data.questions.iter_mut().filter(|q| q.expired_at > clock.unix_timestamp && q.id == questionId).collect();

    if (questions.len() > 0) {
        for q in questions {
            if q.answers.contains(&answerId) {
                let index = q.answers.iter().position(|a| *a == answerId).unwrap();
                q.stats[index] += 1;
            }
        }
    }

    let accounts = vec![
        AccountMeta::new(*stake_program_account.key, false),//program account
        AccountMeta::new(*stake_account.key, false),//stake account
        AccountMeta::new_readonly(*token_program_account.key, false),//token program account
        AccountMeta::new(*user_account.key, false),//user account
        AccountMeta::new(*user_solana_account.key, true),//authority account        
    ];

    let mut instruction_data = to_vec(&StakeInstruction{
        amount: 0,
        instruction: crate::data::StakeProgramInstruction::Vote,
        config: None,
    })?;

    let instruction = Instruction {
        program_id: *stake_program_id.key,
        accounts,
        data: instruction_data,
    };    

        
        invoke(
            &instruction,
            &[
                user_solana_account.clone(),
                program_account.clone(),
                user_account.clone(),                
                token_program_account.clone(),
                stake_program_account.clone(),
                stake_account.clone(),                
            ],
        )?;

    remove_expired_questions(data, program_account)
    
}