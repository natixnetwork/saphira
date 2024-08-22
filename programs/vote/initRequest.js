const solanaWeb3 = require('@solana/web3.js');
const splToken = require('@solana/spl-token');
const {getInstruction} = require("./programInstruction");
const {getProgramAccount, getPayer, getProgramId, getProgramAuth, connectionUrl} = require("./constants");
(async () => {
    const connection = new solanaWeb3.Connection(connectionUrl, "confirmed");
    const programSecretKey = getProgramAccount();    
    const programId = getProgramId();
    const auth = getProgramAuth();
    const payer = getPayer();


    const instruction = new solanaWeb3.TransactionInstruction({
        keys: [
            {pubkey: programSecretKey.publicKey, isSigner: false, isWritable: true},            
            {pubkey: auth.publicKey, isSigner: true, isWritable: false},
        ],
        programId,
        data: Buffer.from(getInstruction(0, undefined, undefined, undefined)),
    });


    const transaction = new solanaWeb3.Transaction().add(instruction);
    console.log('sending transaction...')
    const signature = await solanaWeb3.sendAndConfirmTransaction(
        connection,
        transaction,
        [payer, auth]
    );

    console.log(`Transaction confirmed with signature: ${signature}`);
})();
