use borsh::{BorshDeserialize, BorshSerialize};
use crate::{staking_info::Config, ProgramInstruction};


///
/// Instruction structure to interact with this staking program.
/// 
/// # Instruction
/// it can be one of tese values:
/// - Init,
/// - Stake,
/// - UnStake,
/// - UnStakeInstant,
/// - Withdraw,
/// - ChangeInterest,
/// - Pause,
/// - Resume,    
/// - Config
/// 
/// # Amount
/// Some of the instructions like Stake, ChangeInterest, needs this value, for the rest it can be 0
/// 
/// # Config
/// 
/// It is an optional value, it will be used only for Config instruction, to set the config and restrictions
/// 
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Data {
    pub instruction: ProgramInstruction,
    pub amount: u64,
    pub config: Option<Config>
}