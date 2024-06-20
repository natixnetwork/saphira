const solanaWeb3 = require('@solana/web3.js');
const splToken = require('@solana/spl-token');

const borsh = require('borsh');
const {getProgramAccount} = require("./constants");

const InfoSchema = {
    struct: {
        address: {
            array: {
                type: 'u8',
                len: 32
            }
        },
        amount: 'u64',
        stake_time: 'i64',
        unstake_time: 'i64',
        instant: 'bool'
    }
}

const RateSchema = {
    struct: {
        amount: 'f64',
        time: 'i64',
    }
}

const Schema = {
    struct: {
        stakers: {
            array: {
                type: InfoSchema,
            }
        },
        pause: 'bool',
        interests: {
            array: {
                type: RateSchema,
            }
        },
        config: {
            struct: {
                min_stake_amount: "u64",
                max_stake_amount: "u64",
                cooldown: "f64",
                instant_penalty: "f64",
                maximum_stakers: "u64",
                compound_period: "u16"
            }
        },
        forfeit_amount: "u64",
        provisioned: "bool",
    }
};


(async () => {
    const connection = new solanaWeb3.Connection("https://api.devnet.solana.com", "confirmed");
    const programAccount = getProgramAccount().publicKey;

    const accountInfo = await connection.getAccountInfo(programAccount);

    const stakePool = borsh.deserialize(Schema, accountInfo.data, false);

    console.log(stakePool);

})();
