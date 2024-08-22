use solana_program::program_error::ProgramError;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub enum ProgramInstruction {
    Init,
    Stake,
    UnStake,
    UnStakeInstant,
    Withdraw,
    ChangeInterest,
    Pause,
    Resume,    
    Config,
    Cancel,
    Vote,
    BurnOrTransferForfeit
}
