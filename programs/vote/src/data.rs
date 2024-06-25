use borsh::{BorshDeserialize, BorshSerialize};
use crate::{ProgramInstruction};


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Input {
    pub instruction: ProgramInstruction,
    pub questionId: Option<String>,
    pub answerId: Option<String>,   
    pub question: Option<Question>,
}


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Question {    
    pub id: String,
    pub answers: Vec<String>,
    pub stats: Vec<u64>,   
    pub expired_at: i64, 
}


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Data {    
    pub questions: Vec<Question>,
    pub provisioned: bool,
}



#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct StakeInstruction {
    pub instruction: StakeProgramInstruction,
    pub amount: u64,
    pub config: Option<StakeConfig>
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub enum StakeProgramInstruction {
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
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct StakeConfig {
    pub min_stake_amount: u64,
    pub max_stake_amount: u64,    
    pub cooldown: f64,
    pub instant_penalty: f64,
    pub maximum_stakers: usize,
    pub compound_period: u16,
    pub vote_period_based_on_compound: u16,
}