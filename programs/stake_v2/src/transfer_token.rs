use crate::control_stake_amount::control_stake_amount;
use crate::control_user_account::control_user_account;
use crate::get_natix_token_id::get_natix_token_id;
use crate::get_natix_token_mint::get_natix_token_mint;
use crate::get_pda;
use crate::get_pda::get_pda;
use crate::serialize::serialize;
use crate::staking_info::{StakeInfo, StakePool};
use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;
use spl_token::instruction::{transfer, burn};
use spl_token::state::Account as TokenAccount;

///
/// Transfer token from sender account to receiver account
/// 
pub fn transfer_token<'a>(
    sender: &'a AccountInfo<'a>,
    authority_account: &'a AccountInfo<'a>,
    receiver: &'a AccountInfo<'a>,
    token_program_account: &'a AccountInfo<'a>,
    token_amount: u64,
) -> ProgramResult {
    msg!(
        "Transferring token {:?} from {:?} to {:?}, with authority: {:?}",
        token_program_account.key,
        sender.key,
        receiver.key,
        authority_account.key
    );

    if sender.key == receiver.key {
        return Ok(());
    }

    let transfer_instruction = transfer(
        &get_natix_token_id(),
        sender.key,        
        receiver.key,
        authority_account.key,
        &[authority_account.key],
        token_amount,
    )?;
    
    invoke(
        &transfer_instruction,
        &[
            sender.clone(),
            receiver.clone(),
            authority_account.clone(),
            token_program_account.clone()
        ],
    )
}


///
/// Transfering from stake account to users' account with pda authority
/// 
pub fn transfer_token_pda<'a>(
    sender: &'a AccountInfo<'a>,
    receiver: &'a AccountInfo<'a>,
    token_program_account: &'a AccountInfo<'a>,
    token_amount: u64,
    pda_account: &'a AccountInfo<'a>,
    program_id: &'a Pubkey,
) -> ProgramResult {
    let (pda, bump_seed) = get_pda(program_id, pda_account)?;
    let seeds: &[&[u8]] = &[b"NATIX", &[bump_seed]];
    msg!(
        "Transferring token {:?} from {:?} to {:?}",
        get_natix_token_id(),
        sender.key,
        receiver.key
    );

    if sender.key == receiver.key {
        return Ok(());
    }

    let transfer_instruction = transfer(
        &get_natix_token_id(),
        sender.key,
        receiver.key,
        &pda,
        &[&pda],
        token_amount,
    )?;

    invoke_signed(
        &transfer_instruction,
        &[
            sender.clone(),
            receiver.clone(),
            pda_account.clone(),
            token_program_account.clone(),
        ],
        &[seeds],
    )
}


pub fn burn_token_pda<'a>(
    sender: &'a AccountInfo<'a>,    
    mint_info: &'a AccountInfo<'a>,
    token_program_account: &'a AccountInfo<'a>,    
    token_amount: u64,
    pda_account: &'a AccountInfo<'a>,
    program_id: &'a Pubkey,
) -> ProgramResult {
    let (pda, bump_seed) = get_pda(program_id, pda_account)?;
    let seeds: &[&[u8]] = &[b"NATIX", &[bump_seed]];
    msg!(
        "Burning token {:?} from {:?}",
        get_natix_token_id(),
        sender.key,        
    );
    
    if *mint_info.key != get_natix_token_mint() || token_amount == 0 {
        return Err(ProgramError::InvalidArgument);
    }

    let burn_instruction = burn(
        &get_natix_token_id(),
        sender.key,
        &get_natix_token_mint(),
        &pda,
        &[&pda],
        token_amount,
    )?;

    invoke_signed(
        &burn_instruction,
        &[
            sender.clone(),  
            mint_info.clone(),          
            pda_account.clone(),
            token_program_account.clone(),
        ],
        &[seeds],
    )
}