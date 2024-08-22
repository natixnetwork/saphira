mod tests {
    use solana_program::pubkey::Pubkey;
    use stake::{get_reward::get_reward, staking_info::{Config, Rate, StakeInfo, StakePool}};

    use super::*;

    #[test]
    fn get_reward_test() {
        let pubKey = Pubkey::new_unique();
        let day = (60.0 * 60.0 * 24.0) as i64;
        let mut unstaker = StakeInfo {
            address: pubKey.to_bytes(),
            amount: 10000,
            instant: false,
            stake_time: 1 * day,
            unstake_time: 20 * day
        };

        let staking_pool = StakePool {
            provisioned: true,
            forfeit_amount: 0,
            config: Config {
                compound_period: 7,
                cooldown: 90.0,
                instant_penalty: 0.3,
                max_stake_amount: 100,
                maximum_stakers: 100,
                min_stake_amount: 0,
                forfeit_allow_burn: false,
                forfeit_allow_transfer: false,
            },
            pause: false,
            interests: vec![
                Rate {
                    amount: 0.01,
                    time: 0 * day
                },
                Rate {
                    amount: 0.02,
                    time: 7 * day
                },
                Rate {
                    amount: 0.01,
                    time: 14 * day
                }
            ],
            stakers: vec![]
        };
      
        
        assert_eq!(get_reward(&staking_pool, &unstaker), 5);

        unstaker.unstake_time = 21 * day;

        assert_eq!(get_reward(&staking_pool, &unstaker), 7);


        
    }

}