const solanaWeb3 = require('@solana/web3.js');
const splToken = require('@solana/spl-token');

const borsh = require('borsh');


const Schema = {
    struct: {
        instruction: "u8",
        amount: 'u64',
        config: {
            option: {
                struct: {
                    min_stake_amount: "u64",
                    max_stake_amount: "u64",
                    cooldown: "f64",
                    instant_penalty: "f64",
                    maximum_stakers: "u64",
                    compound_period: "u16",
                }
            }
        }
    }
};


function getInstruction(instruction, amount, config) {
    return borsh.serialize(Schema, {instruction, amount, config});
}

module.exports = {
    getInstruction,
}