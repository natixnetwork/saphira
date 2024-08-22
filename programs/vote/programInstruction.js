const solanaWeb3 = require('@solana/web3.js');
const splToken = require('@solana/spl-token');

const borsh = require('borsh');


const Schema = {
    struct: {
        instruction: "u8",
        questionId: {option: 'string'},
        answerId: {option: 'string'},
        question: {
            option: {
                struct: {
                    id: "string",
                    answers: {array: {type: "string"}},
                    stats: {array: {type: "u64"}},
                    expired_at: "i64",                    
                }
            }
        }
    }
};


function getInstruction(instruction, questionId, answerId, question) {
    return borsh.serialize(Schema, {instruction, questionId, answerId, question});
}

module.exports = {
    getInstruction,
}