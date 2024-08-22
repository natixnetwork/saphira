const solanaWeb3 = require('@solana/web3.js');
const splToken = require('@solana/spl-token');

const borsh = require('borsh');
const {getProgramAccount, connectionUrl} = require("./constants");

const QuestionSchema = {
    struct: {
        id: "string",
        answers: {array: {type: "string"}},
        stats: {array: {type: "u64"}},
        expired_at: "i64",                    
    }
}

const Schema = {
    struct: {
        questions: {
            array: {
                type: QuestionSchema,
            }
        },
        
        provisioned: "bool",
    }
};


(async () => {
    const connection = new solanaWeb3.Connection(connectionUrl, "confirmed");
    const programAccount = getProgramAccount().publicKey;

    const accountInfo = await connection.getAccountInfo(programAccount);

    const stakePool = borsh.deserialize(Schema, accountInfo.data, false);

    console.log(stakePool);

})();
