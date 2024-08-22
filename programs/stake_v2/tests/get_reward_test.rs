mod tests {
    use solana_program::pubkey::Pubkey;
    use stake_v2::{get_reward::{clamped, get_reward}, staking_info::{Config, Rate, StakeInfo, StakePool}, unstake::unstake_staker, vote::set_stake_vote};

    use super::*;

    #[test]
    fn get_reward_test() {
        let pubKey = Pubkey::new_unique();
        let day = (60.0 * 60.0 * 24.0) as i64;
        let mut unstaker = StakeInfo {
            address: pubKey.to_bytes(),
            amount: 10000,
            instant: false,
            stake_time: 0 * day,
            unstake_time: 20 * day,            
            potential_reward: 0,            
            potential_reward_time: 0,
            potential_max_reward_time: 0,
            vote_time: 0,
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
                vote_period_based_on_compound: 3,
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
      
        
        assert_eq!(get_reward(&staking_pool.interests, &staking_pool.config, unstaker.stake_time, unstaker.unstake_time, unstaker.amount), 5);

        unstaker.unstake_time = 21 * day;

        assert_eq!(get_reward(&staking_pool.interests, &staking_pool.config, unstaker.stake_time, unstaker.unstake_time, unstaker.amount), 7);


        
    }




    #[test]
    fn get_vote_reward_test() {
        let pubKey = Pubkey::new_unique();
        let day = (60.0 * 60.0 * 24.0) as i64;
        let mut staker = StakeInfo {
            address: pubKey.to_bytes(),
            amount: 10000,
            instant: false,
            stake_time: 0 * day,
            unstake_time: 0,            
            potential_reward: 0,            
            potential_reward_time: 0,
            potential_max_reward_time: 0,
            vote_time: 0,
        };

        let config = Config {
            compound_period: 7,
            cooldown: 90.0,
            instant_penalty: 0.3,
            max_stake_amount: 100,
            maximum_stakers: 100,
            min_stake_amount: 0,
            vote_period_based_on_compound: 3,
            forfeit_allow_burn: false,
            forfeit_allow_transfer: false,
        };
        let interests = vec![
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
        ];        
                

        unstake_staker(vec![&mut staker], 7 * day, false, &config, &interests);        
        assert_eq!(staker.potential_reward, 0);

        
        set_stake_vote(vec![&mut staker], 9 * day, &config, &interests, true);
        assert_eq!(staker.potential_reward, 1);

    

        set_stake_vote(vec![&mut staker], 10 * day, &config, &interests, true);
        assert_eq!(staker.potential_reward, 1);
    

        unstake_staker(vec![&mut staker], 70 * day, false, &config, &interests);
        assert_eq!(staker.potential_reward, 6);

        unstake_staker(vec![&mut staker], 90 * day, false, &config, &interests);
        assert_eq!(staker.potential_reward, 6);


        
        set_stake_vote(vec![&mut staker], 91 * day, &config, &interests, true);
        assert_eq!(staker.potential_reward, 6);


        set_stake_vote(vec![&mut staker], 101 * day, &config, &interests, true);
        assert_eq!(staker.potential_reward, 7);

        unstake_staker(vec![&mut staker], 102 * day, false, &config, &interests);
        assert_eq!(staker.potential_reward, 7);

        unstake_staker(vec![&mut staker], 109 * day, false, &config, &interests);
        assert_eq!(staker.potential_reward, 8);
        


        
    }

}