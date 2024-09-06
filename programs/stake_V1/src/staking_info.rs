use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use crate::ProgramInstruction;

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct StakeInfo {
    pub address: [u8; 32],
    pub amount: u64,    
    pub stake_time: i64,
    pub unstake_time: i64,
    pub instant: bool
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct Rate {
    pub amount: f64,
    pub time: i64,
}

///
/// Stake Pool struction is the whole structure being saved in the account. 
/// It consists:
/// stakers: a vector of StakeInfo which stores the address of the user, the amount, the stake time, the unstake time and if this request is instant or not
/// pause: which determins if this contract accept any requests or not.
/// interests: A vector of Rate which stores the amount and the time that interest rate has been set.
/// config: Which has a struction to set all the restrictions.
///
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct StakePool {
    pub stakers: Vec<StakeInfo>,
    pub pause: bool,
    pub interests: Vec<Rate>,
    pub config: Config,
    pub forfeit_amount: u64,
    pub provisioned: bool
}

///
/// min_stake_amount: each stake request should be at least this much amount
/// max_stake_amount: sum of all requests per address can not exceed this amount
/// cooldown: if the request is not instant, this many days should be passed until that address can withdraw the staked amount
/// instant_penalty: it's the penalty that user would pay to instantly withdraw, it's in the form of 0.3 for example if it's going to be set for 30 percent.
/// maximus_staker: the count of distinct addresses of stake requests can't exceed this amount
/// compound_period: the amount of days after which we calculate the compound interest for each request.
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct Config {
    pub min_stake_amount: u64,
    pub max_stake_amount: u64,    
    pub cooldown: f64,
    pub instant_penalty: f64,
    pub maximum_stakers: usize,
    pub compound_period: u16,
    pub forfeit_allow_burn: bool,
    pub forfeit_allow_transfer: bool,
    pub stake_account: Option<String>,
    pub program_account: Option<String>
}